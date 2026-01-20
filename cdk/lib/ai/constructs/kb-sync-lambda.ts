import * as iam from "aws-cdk-lib/aws-iam";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as events from "aws-cdk-lib/aws-events";
import * as targets from "aws-cdk-lib/aws-events-targets";
import { Duration } from "aws-cdk-lib";
import { Construct } from "constructs";
import { readFileSync } from "fs";

export interface KbSyncLambdaProps {
  /**
   * The ID of the Knowledge Base
   */
  readonly knowledgeBaseId: string;

  /**
   * The ARN of the Knowledge Base
   */
  readonly knowledgeBaseArn: string;

  /**
   * The ID of the Data Source
   */
  readonly dataSourceId: string;

  /**
   * The name of the S3 bucket containing data
   */
  readonly dataSourceBucketName: string;

  /**
   * The prefix filter for S3 objects (optional)
   */
  readonly dataSourcePrefix?: string;
}

/**
 * Creates a Lambda function that triggers Knowledge Base ingestion
 * when objects are uploaded to S3, along with EventBridge rule.
 */
export class KbSyncLambda extends Construct {
  /**
   * The Lambda function
   */
  public readonly lambdaFunction: lambda.Function;

  /**
   * The EventBridge rule
   */
  public readonly eventRule: events.Rule;

  constructor(scope: Construct, id: string, props: KbSyncLambdaProps) {
    super(scope, id);

    // Create the Lambda function
    this.lambdaFunction = new lambda.Function(this, "SyncTrigger", {
      runtime: lambda.Runtime.NODEJS_22_X,
      handler: "index.handler",
      code: lambda.Code.fromInline(
        readFileSync(__dirname + "/./kb-sync-lambda.inner.js", "utf-8"),
      ),
      environment: {
        KNOWLEDGE_BASE_ID: props.knowledgeBaseId,
        DATA_SOURCE_ID: props.dataSourceId,
        DATA_PREFIX: props.dataSourcePrefix || "",
      },
      timeout: Duration.seconds(30),
      description: `Triggers direct PDF ingestion for ${props.knowledgeBaseId} (CUSTOM data source)`,
    });

    // Grant Lambda permission to ingest documents directly (CUSTOM data source)
    this.lambdaFunction.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: [
          "bedrock:IngestKnowledgeBaseDocuments",
          "bedrock:StartIngestionJob",
        ],
        resources: [props.knowledgeBaseArn],
      }),
    );

    // Grant Lambda permission to read from S3 bucket
    this.lambdaFunction.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: [
          "s3:GetObject",
          "s3:HeadObject",
        ],
        resources: [`arn:aws:s3:::${props.dataSourceBucketName}/${props.dataSourcePrefix}*`],
      }),
    );

    // Create EventBridge rule to capture S3 object created events
    this.eventRule = new events.Rule(this, "S3ObjectCreatedRule", {
      eventPattern: {
        source: ["aws.s3"],
        detailType: ["Object Created"],
        detail: {
          bucket: {
            name: [props.dataSourceBucketName],
          },
          ...(props.dataSourcePrefix && {
            object: {
              key: [{ prefix: props.dataSourcePrefix }],
            },
          }),
        },
      },
      description: `Triggers direct PDF ingestion when PDFs are created in s3://${props.dataSourceBucketName}${props.dataSourcePrefix ? "/" + props.dataSourcePrefix : ""} (CUSTOM data source - no full scans)`,
    });

    this.eventRule.addTarget(new targets.LambdaFunction(this.lambdaFunction));
  }
}
