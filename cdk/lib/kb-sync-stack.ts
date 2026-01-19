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

    // Lambda function to trigger KB direct ingestion on S3 PDF upload
    const syncTrigger = new lambda.Function(this, "KbSyncTrigger", {
      runtime: lambda.Runtime.NODEJS_22_X,
      handler: "index.handler",
      code: lambda.Code.fromInline(`
const { BedrockAgentClient, IngestKnowledgeBaseDocumentsCommand } = require("@aws-sdk/client-bedrock-agent");
const { S3Client, HeadObjectCommand } = require("@aws-sdk/client-s3");

exports.handler = async (event) => {
  const bedrockClient = new BedrockAgentClient({ region: process.env.AWS_REGION });
  const s3Client = new S3Client({ region: process.env.AWS_REGION });
  
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
  
  // Check file extension OR ContentType to verify it's a PDF
  const isPdfExtension = key.toLowerCase().endsWith('.pdf');
  let isPdfContentType = false;
  
  if (!isPdfExtension) {
    try {
      // Files uploaded without extension - check ContentType metadata
      const headResult = await s3Client.send(new HeadObjectCommand({ Bucket: bucket, Key: key }));
      isPdfContentType = headResult.ContentType === 'application/pdf';
      console.log(\`ContentType: \${headResult.ContentType}, is PDF: \${isPdfContentType}\`);
    } catch (error) {
      console.error('Error checking file metadata:', error);
      return { statusCode: 500, body: 'Error checking file type' };
    }
  }
  
  if (!isPdfExtension && !isPdfContentType) {
    console.log(\`Skipping non-PDF file: \${key}\`);
    return { statusCode: 200, body: 'Skipped - not a PDF file' };
  }
  
  console.log('Triggering direct document ingestion for PDF (CUSTOM data source)');
  
  try {
    const s3Uri = \`s3://\${bucket}/\${key}\`;
    
    // Use IngestKnowledgeBaseDocuments for CUSTOM data sources
    const response = await bedrockClient.send(new IngestKnowledgeBaseDocumentsCommand({
      knowledgeBaseId: process.env.KNOWLEDGE_BASE_ID,
      dataSourceId: process.env.DATA_SOURCE_ID,
      documents: [
        {
          content: {
            dataSourceType: 'CUSTOM',
            custom: {
              customDocumentIdentifier: {
                id: key
              },
              sourceType: 'S3_LOCATION',
              s3Location: {
                uri: s3Uri
              }
            }
          }
        }
      ]
    }));
    
    console.log('Direct document ingestion completed:', {
      failedDocuments: response.failedDocuments?.length || 0,
      ingestionStatus: response.documentDetails || []
    });
    
    if (response.failedDocuments && response.failedDocuments.length > 0) {
      console.error('Failed documents:', response.failedDocuments);
      return {
        statusCode: 500,
        body: JSON.stringify({
          message: 'Document ingestion failed',
          errors: response.failedDocuments
        })
      };
    }
    
    return {
      statusCode: 200,
      body: JSON.stringify({
        message: 'Direct document ingestion successful',
        documentUri: s3Uri,
        details: response.documentDetails
      }),
    };
  } catch (error) {
    console.error('Error ingesting document:', error);
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
      description: `Triggers direct PDF ingestion for ${knowledgeBaseId} (CUSTOM data source)`,
    });

    // Grant Lambda permission to ingest documents directly (CUSTOM data source)
    syncTrigger.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: [
          "bedrock:IngestKnowledgeBaseDocuments",
          "bedrock:StartIngestionJob"
        ],
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
      description: `Triggers direct PDF ingestion when PDFs are created in s3://${dataBucketName}${dataPrefix ? '/' + dataPrefix : ''} (CUSTOM data source - no full scans)`,
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
