import {
  aws_apigatewayv2 as apigw,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_route53_targets as targets,
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_servicediscovery as sd,
  aws_logs as logs,
} from "aws-cdk-lib";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";
import {
  HttpLambdaIntegration,
  HttpServiceDiscoveryIntegration,
} from "aws-cdk-lib/aws-apigatewayv2-integrations";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import { Repository } from "aws-cdk-lib/aws-ecr";

import {
  Duration,
  Stack,
  StackProps,
  aws_lambda as lambda,
  aws_iam as iam,
  aws_events as events,
  aws_events_targets as eventsTargets,
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as lambdaEventSources from "aws-cdk-lib/aws-lambda-event-sources";

export interface RegionalLambdaStackProps extends StackProps {
  stage: string;
  // Health check path for ALB target group
  commit: string;

  apiDomain: string;
  baseDomain: string;
}

export class RegionalLambdaStack extends Stack {
  readonly lambdaFunction: lambda.Function;

  constructor(scope: Construct, id: string, props: RegionalLambdaStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { apiDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    // Default VPC + DynamoDB gateway endpoint
    const vpc = ec2.Vpc.fromLookup(this, "DefaultVpc", { isDefault: true });

    const lambdaSg = new ec2.SecurityGroup(this, "LambdaSG", {
      vpc,
      description: "Security group for Regional Lambda",
      allowAllOutbound: true,
    });

    new ec2.GatewayVpcEndpoint(this, "DynamoDbEndpoint", {
      vpc,
      service: ec2.GatewayVpcEndpointAwsService.DYNAMODB,
    });

    // --- HTTP API (shared between ECS and Lambda) ---
    const httpApi = new apigw.HttpApi(this, "HttpApi", {
      apiName: `ratel-api-${this.stackName}`,
      description: "Ratel API Gateway",
    });

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
      vpc,
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC },
      securityGroups: [lambdaSg],
    });
    this.lambdaFunction = apiLambda;

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
