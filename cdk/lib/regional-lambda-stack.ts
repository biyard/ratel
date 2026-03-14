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
}

export class RegionalLambdaStack extends Stack {
  constructor(scope: Construct, id: string, props: RegionalLambdaStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

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
      },
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
      tableName,
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
      }),
    );

    if (this.region === "ap-northeast-2") {
      const mainTableStreamArn = cdk.Fn.importValue(
        `ratel-${props.stage}-main-stream-arn`,
      );
      const mainTableWithStream = dynamodb.Table.fromTableAttributes(
        this,
        "MainTableWithStream",
        {
          tableName,
          tableStreamArn: mainTableStreamArn,
        },
      );

      const spaceStreamLambda = new lambda.Function(this, "SpaceStreamWorker", {
        runtime: lambda.Runtime.PROVIDED_AL2023,
        code: lambda.Code.fromAsset("space-stream-worker"),
        handler: "bootstrap",
        environment: {
          REGION: this.region,
          DISABLE_ANSI: "true",
          NO_COLOR: "true",
        },
        memorySize: 256,
        timeout: Duration.seconds(150),
      });

      const privateBucketName =
        process.env.PRIVATE_BUCKET_NAME ?? "metadata.ratel.foundation";
      if (privateBucketName) {
        spaceStreamLambda.addToRolePolicy(
          new iam.PolicyStatement({
            effect: iam.Effect.ALLOW,
            actions: ["s3:GetObject", "s3:HeadObject", "s3:PutObject"],
            resources: [`arn:aws:s3:::${privateBucketName}/*`],
          }),
        );
        spaceStreamLambda.addToRolePolicy(
          new iam.PolicyStatement({
            effect: iam.Effect.ALLOW,
            actions: ["s3:ListBucket"],
            resources: [`arn:aws:s3:::${privateBucketName}`],
          }),
        );
      }

      spaceStreamLambda.addEventSource(
        new lambdaEventSources.DynamoEventSource(mainTableWithStream, {
          startingPosition: lambda.StartingPosition.LATEST,
          batchSize: 10,
          bisectBatchOnError: true,
          retryAttempts: 3,
        }),
      );
    }

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
      }),
    );
  }
}
