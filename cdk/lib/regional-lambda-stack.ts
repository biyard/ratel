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
  runtimeEnvironment?: { [key: string]: string };

  // Optional — when provided, the Lambda is placed in this VPC (required to
  // resolve CloudMap private DNS for Qdrant). Use the same VPC that hosts Qdrant.
  vpc?: ec2.IVpc;
  // Optional — shared SG from EcsClusterStack. Must be provided whenever vpc is.
  sharedSecurityGroup?: ec2.ISecurityGroup;
}

export class RegionalLambdaStack extends Stack {
  readonly lambdaFunction: lambda.Function;

  constructor(scope: Construct, id: string, props: RegionalLambdaStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { apiDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    // VPC attachment is opt-in. When props.vpc is provided, the Lambda joins
    // the VPC and uses the shared SG from VpcServiceStack (which grants
    // access to Qdrant via self-reference and Bedrock via the endpoint SG).
    // VPC-wide endpoints (DynamoDB, Bedrock) live in VpcServiceStack.
    if (props.vpc && !props.sharedSecurityGroup) {
      throw new Error(
        "RegionalLambdaStack: sharedSecurityGroup is required when vpc is set",
      );
    }

    // --- HTTP API (shared between ECS and Lambda) ---
    const httpApi = new apigw.HttpApi(this, "HttpApi", {
      apiName: `ratel-api-${this.stackName}`,
      description: "Ratel API Gateway",
    });

    const environment: { [key: string]: string } = {
      REGION: this.region,
      DISABLE_ANSI: "true",
      NO_COLOR: "true",
      GOOGLE_APPLICATION_CREDENTIALS: ".gcp/firebase-service-account.json",
      ...(props.runtimeEnvironment ?? {}),
    };

    // Bootstrap binary produced by `scripts/building-lambda-bootstrap.sh` and
    // copied into `cdk/.build/app-shell/bootstrap` by the root Makefile rule
    // `cdk/.build/%/bootstrap`. Path is resolved relative to the CDK cwd.
    const apiLambda = new lambda.Function(this, "Function", {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      architecture: lambda.Architecture.X86_64,
      code: lambda.Code.fromAsset(".build/app-shell"),
      handler: "bootstrap",
      environment,
      memorySize: 128,
      timeout: cdk.Duration.seconds(30),
      // VPC attachment only when a VPC is supplied; otherwise the function
      // runs on the public Lambda fleet (no ENI, faster cold start).
      ...(props.vpc && props.sharedSecurityGroup
        ? {
            allowPublicSubnet: true,
            vpc: props.vpc,
            vpcSubnets: {
              subnetType: ec2.SubnetType.PUBLIC,
              availabilityZones: [
                "ap-northeast-2a",
                "ap-northeast-2b",
                "ap-northeast-2c",
              ],
            },
            securityGroups: [props.sharedSecurityGroup],
          }
        : {}),
    });
    this.lambdaFunction = apiLambda;

    // Grant Bedrock model invocation permissions. The VPC endpoint (above)
    // only provides network reachability — IAM is still required.
    apiLambda.addToRolePolicy(
      new iam.PolicyStatement({
        actions: [
          "bedrock:InvokeModel",
          "bedrock:InvokeModelWithResponseStream",
        ],
        resources: ["*"],
      }),
    );

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
