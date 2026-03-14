import {
  Duration,
  Stack,
  StackProps,
  aws_lambda as lambda,
  aws_apigatewayv2 as apigw,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_route53_targets as targets,
  aws_iam as iam,
  aws_events as events,
  aws_events_targets as eventsTargets,
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_servicediscovery as sd,
  aws_logs as logs,
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";
import {
  HttpLambdaIntegration,
  HttpServiceDiscoveryIntegration,
} from "aws-cdk-lib/aws-apigatewayv2-integrations";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as lambdaEventSources from "aws-cdk-lib/aws-lambda-event-sources";
import { Repository } from "aws-cdk-lib/aws-ecr";

export interface RegionalServiceStackProps extends StackProps {
  // Domain parts, e.g. "dev2.ratel.foundation"
  fullDomainName: string;
  stage: string;
  // Health check path for ALB target group
  healthCheckPath?: string;
  commit: string;
  // Repository names
  apiRepoName?: string;
  webRepoName?: string;
  minCapacity?: number;
  maxCapacity?: number;
  enableDaemon?: boolean;
  pghost: string;

  apiDomain: string;
  baseDomain: string;

  // ECS deployment (high-traffic regions)
  enableEcs?: boolean;
  cluster?: ecs.ICluster;
  vpc?: ec2.IVpc;
}

export class RegionalServiceStack extends Stack {
  constructor(scope: Construct, id: string, props: RegionalServiceStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { apiDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    // --- HTTP API (shared between ECS and Lambda) ---
    const httpApi = new apigw.HttpApi(this, "HttpApi", {
      apiName: `ratel-api-${this.stackName}`,
      description: "Ratel API Gateway",
    });

    if (props.enableEcs && props.cluster && props.vpc) {
      // --- ECS Fargate deployment (high-traffic region) ---
      const { cluster, vpc } = props;

      const appShellRepository = Repository.fromRepositoryName(
        this,
        "AppShellRepository",
        "ratel/app-shell",
      );

      const sg = new ec2.SecurityGroup(this, "AppShellSG", {
        vpc,
        description: "App Shell ECS security group",
        allowAllOutbound: true,
      });
      sg.addIngressRule(
        ec2.Peer.ipv4(vpc.vpcCidrBlock),
        ec2.Port.tcp(8080),
        "App Shell HTTP",
      );

      const taskExecutionRole = new iam.Role(
        this,
        "AppShellTaskExecutionRole",
        {
          assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
        },
      );
      taskExecutionRole.addManagedPolicy(
        iam.ManagedPolicy.fromAwsManagedPolicyName(
          "service-role/AmazonECSTaskExecutionRolePolicy",
        ),
      );

      const taskDefinition = new ecs.TaskDefinition(this, "AppShellTaskDef", {
        compatibility: ecs.Compatibility.FARGATE,
        cpu: "256",
        memoryMiB: "512",
        executionRole: taskExecutionRole,
      });

      const container = taskDefinition.addContainer("AppShellContainer", {
        image: ecs.ContainerImage.fromEcrRepository(
          appShellRepository,
          props.commit,
        ),
        logging: new ecs.AwsLogDriver({
          streamPrefix: `ratel-${props.stage}-app-shell`,
          logRetention: logs.RetentionDays.TWO_WEEKS,
        }),
        environment: {
          REGION: this.region,
          DISABLE_ANSI: "true",
          NO_COLOR: "true",
          GOOGLE_APPLICATION_CREDENTIALS: ".gcp/firebase-service-account.json",
          IP: "0.0.0.0",
          PORT: "8080",
        },
      });

      container.addPortMappings({
        containerPort: 8080,
        protocol: ecs.Protocol.TCP,
      });

      const namespace = new sd.PrivateDnsNamespace(this, "AppShellNamespace", {
        name: `ratel-${props.stage}-svc.local`,
        vpc,
      });

      const fargateService = new ecs.FargateService(this, "AppShellService", {
        cluster,
        taskDefinition,
        desiredCount: props.minCapacity ?? 2,
        maxHealthyPercent: 200,
        minHealthyPercent: 100,
        assignPublicIp: true,
        vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC },
        securityGroups: [sg],
        cloudMapOptions: {
          name: "app-shell",
          cloudMapNamespace: namespace,
          dnsRecordType: sd.DnsRecordType.A,
          container,
          containerPort: 8080,
        },
      });

      const supportedSubnets = vpc.publicSubnets.filter(
        (s) => s.availabilityZone !== "ap-northeast-2d",
      );

      const vpcLink = new apigw.VpcLink(this, "AppShellVpcLink", {
        vpc,
        subnets: { subnets: supportedSubnets },
        securityGroups: [sg],
      });

      const ecsIntegration = new HttpServiceDiscoveryIntegration(
        "EcsIntegration",
        fargateService.cloudMapService!,
        { vpcLink },
      );

      httpApi.addRoutes({
        path: "/{proxy+}",
        methods: [apigw.HttpMethod.ANY],
        integration: ecsIntegration,
      });
      httpApi.addRoutes({
        path: "/",
        methods: [apigw.HttpMethod.ANY],
        integration: ecsIntegration,
      });
    } else {
      // --- Lambda deployment (default) ---
      const appShellRepository = Repository.fromRepositoryName(
        this,
        "AppShellRepository",
        "ratel/app-shell-lambda",
      );

      const apiLambda = new lambda.DockerImageFunction(this, "Function", {
        code: lambda.DockerImageCode.fromEcr(appShellRepository, {
          tagOrDigest: props.commit,
        }),
        environment: {
          REGION: this.region,
          DISABLE_ANSI: "true",
          NO_COLOR: "true",
          GOOGLE_APPLICATION_CREDENTIALS: ".gcp/firebase-service-account.json",
        },
        memorySize: 128,
        timeout: cdk.Duration.seconds(30),
      });

      const lambdaIntegration = new HttpLambdaIntegration(
        "LambdaIntegration",
        apiLambda,
      );

      httpApi.addRoutes({
        path: "/{proxy+}",
        methods: [apigw.HttpMethod.ANY],
        integration: lambdaIntegration,
      });
      httpApi.addRoutes({
        path: "/",
        methods: [apigw.HttpMethod.ANY],
        integration: lambdaIntegration,
      });
    }

    // Certificate for custom domain
    const cert = new acm.Certificate(this, "Cert", {
      domainName: apiDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // Custom domain for API Gateway
    const domainName = new apigw.DomainName(this, "CustomDomain", {
      domainName: apiDomain,
      certificate: cert,
    });

    // API mapping
    new apigw.ApiMapping(this, "ApiMapping", {
      api: httpApi,
      domainName: domainName,
    });

    const region = this.region;
    const rid = region;

    // Latency-based routing for multi-region deployment
    new route53.CfnRecordSet(this, `LatencyA-${rid}`, {
      hostedZoneId: zone.hostedZoneId,
      name: apiDomain,
      type: "A",
      setIdentifier: `apigw-${rid}`,
      region,
      aliasTarget: {
        dnsName: domainName.regionalDomainName,
        hostedZoneId: domainName.regionalHostedZoneId,
        evaluateTargetHealth: false,
      },
    });

    new route53.CfnRecordSet(this, `LatencyAAAA-${rid}`, {
      hostedZoneId: zone.hostedZoneId,
      name: apiDomain,
      type: "AAAA",
      setIdentifier: `apigw6-${rid}`,
      region,
      aliasTarget: {
        dnsName: domainName.regionalDomainName,
        hostedZoneId: domainName.regionalHostedZoneId,
        evaluateTargetHealth: false,
      },
    });

    // Regional domain for debugging/testing
    const d = apiDomain.replace(`.${baseDomain}`, "");
    const regionalDomain = `${this.region}.${d}`;
    new route53.ARecord(this, "RegionalAliasV4", {
      zone: zone,
      recordName: regionalDomain,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.ApiGatewayv2DomainProperties(
          domainName.regionalDomainName,
          domainName.regionalHostedZoneId,
        ),
      ),
    });
    new route53.AaaaRecord(this, "RegionalAliasV6", {
      zone: zone,
      recordName: regionalDomain,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.ApiGatewayv2DomainProperties(
          domainName.regionalDomainName,
          domainName.regionalHostedZoneId,
        ),
      ),
    });
  }
}
