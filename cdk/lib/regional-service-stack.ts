import {
  Duration,
  Stack,
  StackProps,
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_ecs_patterns as ecs_patterns,
  aws_elasticloadbalancingv2 as elbv2,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_cloudfront as cloudfront,
  aws_cloudfront_origins as origins,
  aws_route53_targets as targets,
  aws_iam as iam,
} from "aws-cdk-lib";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { Construct } from "constructs";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";
import { DaemonStack } from "./daemon-stack";

export interface RegionalServiceStackProps extends StackProps {
  // Domain parts, e.g. "dev2.ratel.foundation"
  fullDomainName: string;
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
}

export class RegionalServiceStack extends Stack {
  constructor(scope: Construct, id: string, props: RegionalServiceStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { apiDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });
    const domain = props.fullDomainName;
    const healthPath = props.healthCheckPath ?? "/version";
    const apiRepoName = props.apiRepoName ?? "ratel/main-api";
    const webRepoName = props.webRepoName ?? "ratel/web";
    const minCapacity = props.minCapacity ?? 2;
    const maxCapacity = props.maxCapacity ?? 2;

    const vpc = ec2.Vpc.fromLookup(this, "Vpc", { isDefault: true });
    const cluster = new ecs.Cluster(this, "Cluster", { vpc });

    // 4) Task execution role
    const taskExecutionRole = new iam.Role(this, "TaskExecutionRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
    });

    taskExecutionRole.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "service-role/AmazonECSTaskExecutionRolePolicy",
      ),
    );

    // --- API Task Definition ---
    const apiTaskDefinition = new ecs.TaskDefinition(
      this,
      "ApiTaskDefinition",
      {
        compatibility: ecs.Compatibility.FARGATE,
        cpu: "256",
        memoryMiB: "512",
        executionRole: taskExecutionRole,
      },
    );

    const apiRepository = Repository.fromRepositoryName(
      this,
      "ApiRepository",
      apiRepoName,
    );
    const apiContainer = apiTaskDefinition.addContainer("ApiContainer", {
      image: ecs.ContainerImage.fromEcrRepository(apiRepository, props.commit),
      logging: new ecs.AwsLogDriver({
        streamPrefix: "ratel-api",
      }),
      environment: {
        PGHOST: props.pghost,
        REGION: this.region,
      },
    });

    apiContainer.addPortMappings({
      containerPort: 3000,
      protocol: ecs.Protocol.TCP,
    });

    // --- Web Task Definition ---
    // const webTaskDefinition = new ecs.TaskDefinition(
    //   this,
    //   "WebTaskDefinition",
    //   {
    //     compatibility: ecs.Compatibility.FARGATE,
    //     cpu: "256",
    //     memoryMiB: "512",
    //     executionRole: taskExecutionRole,
    //   },
    // );

    // const webRepository = Repository.fromRepositoryName(
    //   this,
    //   "WebRepository",
    //   webRepoName,
    // );
    // const webContainer = webTaskDefinition.addContainer("WebContainer", {
    //   image: ecs.ContainerImage.fromEcrRepository(webRepository, props.commit),
    //   logging: new ecs.AwsLogDriver({
    //     streamPrefix: "ratel-web",
    //   }),
    //   environment: {
    //     NODE_ENV: "production",
    //     PORT: "8080",
    //     NEXT_PUBLIC_VERSION: props.commit,
    //     NEXT_PUBLIC_LOG_LEVEL: process.env.NEXT_PUBLIC_LOG_LEVEL!,
    //     NEXT_PUBLIC_API_URL: process.env.NEXT_PUBLIC_API_URL!,
    //     NEXT_PUBLIC_SIGN_DOMAIN: process.env.NEXT_PUBLIC_SIGN_DOMAIN!,
    //     NEXT_PUBLIC_GRAPHQL_URL: process.env.NEXT_PUBLIC_GRAPHQL_URL!,
    //     NEXT_PUBLIC_EXPERIMENT: process.env.NEXT_PUBLIC_EXPERIMENT!,
    //   },
    // });

    // webContainer.addPortMappings({
    //   containerPort: 8080,
    //   protocol: ecs.Protocol.TCP,
    // });

    // 6) Fargate Services (separate services for API and Web)
    const apiService = new ecs.FargateService(this, "ApiService", {
      cluster,
      taskDefinition: apiTaskDefinition,
      desiredCount: minCapacity,
      maxHealthyPercent: 200,
      minHealthyPercent: minCapacity === 1 ? 0 : 50,
      assignPublicIp: true,
    });

    // const webService = new ecs.FargateService(this, "WebService", {
    //   cluster,
    //   taskDefinition: webTaskDefinition,
    //   desiredCount: minCapacity,
    //   maxHealthyPercent: 200,
    //   minHealthyPercent: minCapacity === 1 ? 0 : 50,
    //   assignPublicIp: true,
    // });

    // 7) Auto Scaling (separate for each service)
    const apiScaling = apiService.autoScaleTaskCount({
      minCapacity,
      maxCapacity,
    });

    apiScaling.scaleOnCpuUtilization("ApiCpuScaling", {
      targetUtilizationPercent: 70,
      scaleInCooldown: Duration.seconds(60),
      scaleOutCooldown: Duration.seconds(60),
    });

    // const webScaling = webService.autoScaleTaskCount({
    //   minCapacity,
    //   maxCapacity,
    // });

    // webScaling.scaleOnCpuUtilization("WebCpuScaling", {
    //   targetUtilizationPercent: 70,
    //   scaleInCooldown: Duration.seconds(60),
    //   scaleOutCooldown: Duration.seconds(60),
    // });

    // 9) Target Groups
    // Web Target Group (Next.js)
    // const webTargetGroup = new elbv2.ApplicationTargetGroup(
    //   this,
    //   "WebTargetGroup",
    //   {
    //     targets: [
    //       webService.loadBalancerTarget({
    //         containerName: "WebContainer",
    //         containerPort: 8080,
    //       }),
    //     ],
    //     protocol: elbv2.ApplicationProtocol.HTTP,
    //     vpc,
    //     port: 8080,
    //     deregistrationDelay: Duration.seconds(30),
    //     healthCheck: {
    //       path: "/api/version",
    //       interval: Duration.seconds(30),
    //       timeout: Duration.seconds(5),
    //       healthyHttpCodes: "200",
    //       healthyThresholdCount: 2,
    //       unhealthyThresholdCount: 3,
    //     },
    //   },
    // );

    // API Target Group
    const apiTargetGroup = new elbv2.ApplicationTargetGroup(
      this,
      "ApiTargetGroup",
      {
        targets: [
          apiService.loadBalancerTarget({
            containerName: "ApiContainer",
            containerPort: 3000,
          }),
        ],
        protocol: elbv2.ApplicationProtocol.HTTP,
        vpc,
        port: 3000,
        deregistrationDelay: Duration.seconds(30),
        healthCheck: {
          path: healthPath,
          interval: Duration.seconds(30),
          timeout: Duration.seconds(5),
          healthyHttpCodes: "200",
          healthyThresholdCount: 2,
          unhealthyThresholdCount: 3,
        },
      },
    );
    const cert = new acm.Certificate(this, "Cert", {
      domainName: apiDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    const alb = new elbv2.ApplicationLoadBalancer(this, "ALB", {
      vpc,
      internetFacing: true,
    });

    const listener = alb.addListener("HttpsListener", {
      port: 443,
      certificates: [cert],
      open: true,
    });

    listener.addAction("RedirectToHttps", {
      action: elbv2.ListenerAction.redirect({ protocol: "HTTPS", port: "443" }),
    });

    // 10) Listener Rules
    // Default action: forward to web (Next.js)
    // listener.addTargetGroups("TgRuleWebHost", {
    //   targetGroups: [webTargetGroup],
    // });

    // API Target Group
    listener.addTargetGroups("TgRuleApiHost", {
      priority: 10,
      conditions: [
        elbv2.ListenerCondition.pathPatterns(["/v1/*", "/v2/*", "/version"]),
      ],
      targetGroups: [apiTargetGroup],
    });
    const d = apiDomain.replace(`.${baseDomain}`, "");
    const regionalDomain = `${this.region}.${d}`;
    new route53.ARecord(this, "AlbAliasV4", {
      zone: zone,
      recordName: regionalDomain,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.LoadBalancerTarget(alb),
      ),
    });
    new route53.AaaaRecord(this, "AlbAliasV6", {
      zone: zone,
      recordName: regionalDomain,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.LoadBalancerTarget(alb),
      ),
    });

    if (props.enableDaemon) {
      new DaemonStack(this, {
        vpc,
        cluster,
        listener,
        taskExecutionRole,
        commit: props.commit,
      });
    }

    const region = this.region;
    const rid = region;

    new route53.CfnRecordSet(this, `LatencyA-${rid}`, {
      hostedZoneId: zone.hostedZoneId,
      name: apiDomain,
      type: "A",
      setIdentifier: `alb-${rid}`,
      region,
      aliasTarget: {
        dnsName: alb.loadBalancerDnsName,
        hostedZoneId: alb.loadBalancerCanonicalHostedZoneId,
        evaluateTargetHealth: false,
      },
    });

    new route53.CfnRecordSet(this, `LatencyAAAA-${rid}`, {
      hostedZoneId: zone.hostedZoneId,
      name: apiDomain,
      type: "AAAA",
      setIdentifier: `alb6-${rid}`,
      region,
      aliasTarget: {
        dnsName: alb.loadBalancerDnsName,
        hostedZoneId: alb.loadBalancerCanonicalHostedZoneId,
        evaluateTargetHealth: false,
      },
    });
  }
}
