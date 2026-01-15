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
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";
import { HttpLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";

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
}

export class RegionalServiceStack extends Stack {
  constructor(scope: Construct, id: string, props: RegionalServiceStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { apiDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    const region = cdk.Stack.of(this).region;

    const chromiumLayerMapRaw = process.env.CHROMIUM_LAYER_ARNS_JSON ?? "";
    let chromiumLayerArn: string | undefined;

    if (chromiumLayerMapRaw.trim().length > 0) {
      const parsed = JSON.parse(chromiumLayerMapRaw) as Record<string, string>;
      const v = parsed[region];
      if (typeof v === "string" && v.trim().length > 0) {
        chromiumLayerArn = v.trim();
      }
    } else if (
      process.env.CHROMIUM_LAYER_ARN &&
      process.env.CHROMIUM_LAYER_ARN.trim().length > 0
    ) {
      chromiumLayerArn = process.env.CHROMIUM_LAYER_ARN.trim();
    }

    const chromiumLayer = chromiumLayerArn
      ? lambda.LayerVersion.fromLayerVersionArn(
          this,
          "ChromiumLayer",
          chromiumLayerArn
        )
      : undefined;

    const apiLambda = new lambda.Function(this, "Function", {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      code: lambda.Code.fromAsset("main-api"),
      handler: "bootstrap",
      environment: {
        REGION: this.region,
        DISABLE_ANSI: "true",
        NO_COLOR: "true",
        GOOGLE_APPLICATION_CREDENTIALS: ".gcp/firebase-service-account.json",
        ...(chromiumLayerArn
          ? {
              CHROME_PATH: "/opt/headless-chromium/headless-chromium",
              CHROME_BIN: "/opt/headless-chromium/headless-chromium",
              PUPPETEER_EXECUTABLE_PATH:
                "/opt/headless-chromium/headless-chromium",
              HOME: "/tmp",
              FONTCONFIG_PATH: "/opt/headless-chromium/.fontconfig",
              XDG_CACHE_HOME: "/tmp",
              TMPDIR: "/tmp",
            }
          : {
              REPORT_RENDER_DISABLED: "true",
            }),
      },
      memorySize: 1024,
      timeout: cdk.Duration.seconds(120),
      ephemeralStorageSize: cdk.Size.mebibytes(1024),
      architecture: lambda.Architecture.X86_64,
      layers: chromiumLayer ? [chromiumLayer] : [],
    });

    const startSurveyLambda = new lambda.Function(
      this,
      "StartSurveyEventLambda",
      {
        runtime: lambda.Runtime.PROVIDED_AL2023,
        code: lambda.Code.fromAsset("survey-worker"),
        handler: "bootstrap",
        environment: {
          REGION: this.region,
          DISABLE_ANSI: "true",
          NO_COLOR: "true",
        },
        memorySize: 256,
        timeout: Duration.seconds(150),
        architecture: lambda.Architecture.X86_64,
      }
    );

    const eventBus = new events.EventBus(this, "RatelEventBus", {
      eventBusName: `ratel-${props.stage}-bus`,
    });

    const schedulerRole = new iam.Role(this, "SurveySchedulerRole", {
      roleName: `ratel-${props.stage}-${this.region}-survey-scheduler-role`,
      assumedBy: new iam.ServicePrincipal("scheduler.amazonaws.com"),
    });

    eventBus.grantPutEventsTo(schedulerRole);

    new events.Rule(this, "SurveyFetcherRule", {
      eventBus,
      description: "Route Survey Fetcher events to survey fetcher Lambda",
      eventPattern: {
        source: ["ratel.spaces"],
        detailType: ["SurveyFetcher"],
      },
      targets: [new eventsTargets.LambdaFunction(startSurveyLambda)],
    });

    const tableName = `ratel-${props.stage}-main`;

    const mainTable = dynamodb.Table.fromTableName(
      this,
      "MainTable",
      tableName
    );

    mainTable.grantReadData(startSurveyLambda);

    startSurveyLambda.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: [
          "ses:SendEmail",
          "ses:SendRawEmail",
          "ses:SendTemplatedEmail",
          "ses:SendBulkEmail",
          "ses:SendBulkTemplatedEmail",
        ],
        resources: [
          `arn:aws:ses:${this.region}:${this.account}:identity/ratel.foundation`,
          `arn:aws:ses:${this.region}:${this.account}:template/start_survey`,
        ],
      })
    );

    startSurveyLambda.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: [
          "dynamodb:GetItem",
          "dynamodb:BatchGetItem",
          "dynamodb:Query",
          "dynamodb:Scan",
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
          "dynamodb:TransactWriteItems",
        ],
        resources: [mainTable.tableArn, `${mainTable.tableArn}/index/*`],
      })
    );

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
