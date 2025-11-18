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
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";
import { HttpLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import * as apigateway from "aws-cdk-lib/aws-apigateway";

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

    const apiLambda = new lambda.Function(this, "Function", {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      code: lambda.Code.fromAsset("main-api"),
      handler: "bootstrap",
      environment: {
        REGION: this.region,
        DISABLE_ANSI: "true",
        NO_COLOR: "true",
      },
      memorySize: 128,
      timeout: cdk.Duration.seconds(30),
    });

    // const pollEventSchedulerRole = new iam.Role(
    //   this,
    //   "PollEventSchedulerRole",
    //   {
    //     assumedBy: new iam.ServicePrincipal("scheduler.amazonaws.com"),
    //   }
    // );

    // pollEventSchedulerRole.addToPolicy(
    //   new iam.PolicyStatement({
    //     actions: ["lambda:InvokeFunction"],
    //     resources: [apiLambda.functionArn],
    //   })
    // );

    // apiLambda.addEnvironment("POLL_EVENT_LAMBDA_ARN", apiLambda.functionArn);
    // apiLambda.addEnvironment(
    //   "POLL_EVENT_ROLE_ARN",
    //   pollEventSchedulerRole.roleArn
    // );

    // apiLambda.addToRolePolicy(
    //   new iam.PolicyStatement({
    //     actions: [
    //       "scheduler:CreateSchedule",
    //       "scheduler:UpdateSchedule",
    //       "scheduler:DeleteSchedule",
    //       "scheduler:GetSchedule",
    //     ],
    //     resources: ["*"],
    //   })
    // );

    // apiLambda.addToRolePolicy(
    //   new iam.PolicyStatement({
    //     actions: ["iam:PassRole"],
    //     resources: [pollEventSchedulerRole.roleArn],
    //   })
    // );

    // --- API Gateway HTTP API ---
    const httpApi = new apigw.HttpApi(this, "HttpApi", {
      apiName: `ratel-api-${this.stackName}`,
      description: "Ratel API Gateway",
    });

    // Lambda integration
    const lambdaIntegration = new HttpLambdaIntegration(
      "LambdaIntegration",
      apiLambda
    );

    // Add route for all methods and paths
    httpApi.addRoutes({
      path: "/{proxy+}",
      methods: [apigw.HttpMethod.ANY],
      integration: lambdaIntegration,
    });

    // Add root path route
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
          domainName.regionalHostedZoneId
        )
      ),
    });
    new route53.AaaaRecord(this, "RegionalAliasV6", {
      zone: zone,
      recordName: regionalDomain,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.ApiGatewayv2DomainProperties(
          domainName.regionalDomainName,
          domainName.regionalHostedZoneId
        )
      ),
    });
  }
}
