import {
  Stack,
  StackProps,
  aws_iam as iam,
  aws_lambda as lambda,
  aws_s3 as s3,
  aws_s3_notifications as s3n,
  CfnOutput,
  Duration,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export interface BedrockAgentStackProps extends StackProps {
  stage: string;
  knowledgeBaseId: string;
  dataSourceId: string;
  pdfBucketName: string;
}

export class BedrockAgentStack extends Stack {
  constructor(scope: Construct, id: string, props: BedrockAgentStackProps) {
    super(scope, id, props);

    const { stage, knowledgeBaseId, dataSourceId, pdfBucketName } = props;

    // Reference existing S3 bucket
    const pdfBucket = s3.Bucket.fromBucketName(this, "PdfBucket", pdfBucketName);

    // Lambda function to trigger KB ingestion on S3 upload
    const ingestionTrigger = new lambda.Function(this, "IngestionTrigger", {
      runtime: lambda.Runtime.NODEJS_20_X,
      handler: "index.handler",
      code: lambda.Code.fromInline(`
const { BedrockAgentClient, StartIngestionJobCommand } = require("@aws-sdk/client-bedrock-agent");

exports.handler = async (event) => {
  const client = new BedrockAgentClient({ region: process.env.AWS_REGION });
  
  for (const record of event.Records) {
    const key = record.s3.object.key;
    
    // Only process PDF files
    if (!key.endsWith('.pdf')) {
      console.log('Skipping non-PDF file:', key);
      continue;
    }
    
    console.log('Triggering ingestion for:', key);
    
    try {
      await client.send(new StartIngestionJobCommand({
        knowledgeBaseId: process.env.KNOWLEDGE_BASE_ID,
        dataSourceId: process.env.DATA_SOURCE_ID,
      }));
      console.log('Ingestion job started successfully');
    } catch (error) {
      console.error('Error starting ingestion:', error);
      throw error;
    }
  }
};
      `),
      environment: {
        KNOWLEDGE_BASE_ID: knowledgeBaseId,
        DATA_SOURCE_ID: dataSourceId,
      },
      timeout: Duration.seconds(30),
    });

    // Grant Lambda permission to start ingestion jobs
    ingestionTrigger.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["bedrock:StartIngestionJob"],
        resources: [`arn:aws:bedrock:${this.region}:${this.account}:knowledge-base/${knowledgeBaseId}`],
      })
    );

    // Add S3 event notification to trigger Lambda on file upload
    pdfBucket.addEventNotification(
      s3.EventType.OBJECT_CREATED,
      new s3n.LambdaDestination(ingestionTrigger),
      { suffix: ".pdf" }
    );

    new CfnOutput(this, "IngestionLambdaArn", {
      value: ingestionTrigger.functionArn,
      description: "Lambda function that triggers KB ingestion",
    });
  }
}
