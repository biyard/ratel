import {
  Stack,
  StackProps,
  aws_iam as iam,
  aws_lambda as lambda,
  aws_s3 as s3,
  aws_events as events,
  aws_events_targets as targets,
  CfnOutput,
  Duration,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export interface KbSyncStackProps extends StackProps {
  stage: string;
  knowledgeBaseId: string;
  dataSourceId: string;
  dataBucketName: string;
  dataPrefix?: string;
}

export class KbSyncStack extends Stack {
  constructor(scope: Construct, id: string, props: KbSyncStackProps) {
    super(scope, id, props);

    const { stage, knowledgeBaseId, dataSourceId, dataBucketName, dataPrefix } = props;

    // Reference existing S3 bucket
    const dataBucket = s3.Bucket.fromBucketName(this, "DataBucket", dataBucketName);

    // Lambda function to trigger KB ingestion on S3 upload
    const syncTrigger = new lambda.Function(this, "KbSyncTrigger", {
      runtime: lambda.Runtime.NODEJS_22_X,
      handler: "index.handler",
      code: lambda.Code.fromInline(`
const { BedrockAgentClient, StartIngestionJobCommand } = require("@aws-sdk/client-bedrock-agent");

exports.handler = async (event) => {
  const client = new BedrockAgentClient({ region: process.env.AWS_REGION });
  
  console.log('Received EventBridge event:', JSON.stringify(event, null, 2));
  
  const bucket = event.detail?.bucket?.name;
  const key = event.detail?.object?.key;
  
  if (!bucket || !key) {
    console.error('Missing bucket or key in event');
    return { statusCode: 400, body: 'Invalid event' };
  }
  
  console.log(\`Processing file: s3://\${bucket}/\${key}\`);
  
  // Check if file matches the prefix filter (if specified)
  const prefix = process.env.DATA_PREFIX || '';
  if (prefix && !key.startsWith(prefix)) {
    console.log(\`Skipping file outside prefix '\${prefix}': \${key}\`);
    return { statusCode: 200, body: 'Skipped - outside prefix' };
  }
  
  console.log('Triggering ingestion job for Knowledge Base:', process.env.KNOWLEDGE_BASE_ID);
  
  try {
    const response = await client.send(new StartIngestionJobCommand({
      knowledgeBaseId: process.env.KNOWLEDGE_BASE_ID,
      dataSourceId: process.env.DATA_SOURCE_ID,
    }));
    
    console.log('Ingestion job started successfully:', {
      jobId: response.ingestionJob?.ingestionJobId,
      status: response.ingestionJob?.status,
    });
    
    return {
      statusCode: 200,
      body: JSON.stringify({
        message: 'Ingestion triggered successfully',
        jobId: response.ingestionJob?.ingestionJobId,
      }),
    };
  } catch (error) {
    console.error('Error starting ingestion:', error);
    throw error;
  }
};
      `),
      environment: {
        KNOWLEDGE_BASE_ID: knowledgeBaseId,
        DATA_SOURCE_ID: dataSourceId,
        DATA_PREFIX: dataPrefix || '',
      },
      timeout: Duration.seconds(30),
      description: `Triggers KB ingestion on S3 uploads for ${knowledgeBaseId}`,
    });

    // Grant Lambda permission to start ingestion jobs
    syncTrigger.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["bedrock:StartIngestionJob"],
        resources: [
          `arn:aws:bedrock:${this.region}:${this.account}:knowledge-base/${knowledgeBaseId}`,
        ],
      })
    );

    // Grant Lambda permission to read from S3 bucket
    dataBucket.grantRead(syncTrigger);

    // Create EventBridge rule to capture S3 object created events
    const rule = new events.Rule(this, "S3ObjectCreatedRule", {
      eventPattern: {
        source: ["aws.s3"],
        detailType: ["Object Created"],
        detail: {
          bucket: {
            name: [dataBucketName],
          },
          ...(dataPrefix && {
            object: {
              key: [{ prefix: dataPrefix }],
            },
          }),
        },
      },
      description: `Triggers KB sync when objects are created in s3://${dataBucketName}${dataPrefix ? '/' + dataPrefix : ''}`,
    });

    // Add Lambda as target
    rule.addTarget(new targets.LambdaFunction(syncTrigger));

    // Outputs
    new CfnOutput(this, "SyncLambdaArn", {
      value: syncTrigger.functionArn,
      description: "Lambda function that triggers KB ingestion on S3 uploads",
      exportName: `${stage}-kb-sync-lambda-arn`,
    });

    new CfnOutput(this, "SyncLambdaName", {
      value: syncTrigger.functionName,
      description: "Lambda function name",
      exportName: `${stage}-kb-sync-lambda-name`,
    });
  }
}
