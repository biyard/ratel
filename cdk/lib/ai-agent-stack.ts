import * as cdk from "aws-cdk-lib";
import * as bedrock from "aws-cdk-lib/aws-bedrock";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as iam from "aws-cdk-lib/aws-iam";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as events from "aws-cdk-lib/aws-events";
import * as targets from "aws-cdk-lib/aws-events-targets";
import { Construct } from "constructs";
import * as s3Vectors from "./s3-vectors/bucket";
import * as s3VectorsIndex from "./s3-vectors/index";
import * as s3VectorsKB from "./s3-vectors/knowledge-base";

export interface AiAgentStackProps extends cdk.StackProps {
  stage: string;
  knowledgeBaseName: string;
  description: string;
  dataSourceBucketArn: string;
  dataSourceBucketName: string;
  dataSourcePrefix: string;
  dataSourceName: string;
}

export class AiAgentStack extends cdk.Stack {
  public readonly knowledgeBase: s3VectorsKB.KnowledgeBase;
  public readonly dataSource: bedrock.CfnDataSource;
  public readonly agent: bedrock.CfnAgent;
  public readonly agentAlias: bedrock.CfnAgentAlias;
  public readonly syncLambda: lambda.Function;

  constructor(scope: Construct, id: string, props: AiAgentStackProps) {
    super(scope, id, props);

    const { stage, dataSourceBucketName, dataSourcePrefix } = props;

    const kbNameSanitized = props.knowledgeBaseName
      .toLowerCase()
      .replace(/[^a-z0-9-]/g, "-");

    // ========================================
    // Knowledge Base Resources
    // ========================================

    // Create S3 Vectors bucket
    const vectorBucket = new s3Vectors.Bucket(this, "VectorBucket", {
      vectorBucketName: `${kbNameSanitized}-vectors`,
    });

    // Create S3 Vectors index
    const vectorIndex = new s3VectorsIndex.Index(this, "VectorIndex", {
      vectorBucketName: vectorBucket.vectorBucketName,
      indexName: `${kbNameSanitized}-index`,
      dataType: "float32",
      dimension: 1024, // Titan G2 produces 1024-dimensional vectors
      distanceMetric: "cosine",
      metadataConfiguration: {
        nonFilterableMetadataKeys: [
          "AMAZON_BEDROCK_TEXT",
          "AMAZON_BEDROCK_METADATA",
        ],
      },
    });

    vectorIndex.node.addDependency(vectorBucket);

    // Create Knowledge Base with S3 Vectors
    this.knowledgeBase = new s3VectorsKB.KnowledgeBase(this, "KnowledgeBase", {
      knowledgeBaseName: props.knowledgeBaseName,
      description: props.description,
      vectorBucketArn: vectorBucket.vectorBucketArn,
      indexArn: vectorIndex.indexArn,
      knowledgeBaseConfiguration: {
        embeddingModelArn: `arn:aws:bedrock:${this.region}::foundation-model/amazon.titan-embed-text-v2:0`,
        embeddingDataType: "FLOAT32",
        dimensions: "1024",
      },
    });

    this.knowledgeBase.node.addDependency(vectorIndex);
    this.knowledgeBase.node.addDependency(vectorBucket);

    // Get the data source bucket
    const dataSourceBucket = s3.Bucket.fromBucketArn(
      this,
      "DataSourceBucket",
      props.dataSourceBucketArn,
    );

    // Grant read access to the KB role for the data source bucket
    dataSourceBucket.grantRead(this.knowledgeBase.role);

    // Create the S3 Data Source
    this.dataSource = new bedrock.CfnDataSource(this, "DataSource", {
      name: props.dataSourceName,
      description: `S3 data source for ${props.knowledgeBaseName}`,
      knowledgeBaseId: this.knowledgeBase.knowledgeBaseId,
      dataSourceConfiguration: {
        type: "S3",
        s3Configuration: {
          bucketArn: props.dataSourceBucketArn,
          ...(props.dataSourcePrefix && {
            inclusionPrefixes: [props.dataSourcePrefix],
          }),
        },
      },
    });

    this.dataSource.node.addDependency(this.knowledgeBase);
    this.dataSource.node.addDependency(dataSourceBucket);

    // ========================================
    // Bedrock Agent Resources
    // ========================================

    const agentRole = new iam.Role(this, "BedrockAgentRole", {
      assumedBy: new iam.ServicePrincipal("bedrock.amazonaws.com", {
        conditions: {
          StringEquals: {
            "aws:SourceAccount": this.account,
          },
          ArnLike: {
            "aws:SourceArn": `arn:aws:bedrock:${this.region}:${this.account}:agent/*`,
          },
        },
      }),
      description:
        "Role for Bedrock Agent to invoke models and access Knowledge Base",
    });

    // Grant agent permission to invoke Nova Pro model
    agentRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "AmazonBedrockAgentInferenceProfilesCrossRegionPolicyProd",
        effect: iam.Effect.ALLOW,
        actions: [
          "bedrock:InvokeModel",
          "bedrock:InvokeModelWithResponseStream",
          "bedrock:GetInferenceProfile",
          "bedrock:GetFoundationModel",
        ],
        resources: [
          `arn:aws:bedrock:${this.region}:${this.account}:inference-profile/apac.amazon.nova-pro-v1:0`,
          "arn:aws:bedrock:*::foundation-model/amazon.nova-pro-v1:0",
        ],
      }),
    );

    // Grant agent permission to retrieve from Knowledge Base
    agentRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "AmazonBedrockAgentRetrieveKnowledgeBasePolicyProd",
        effect: iam.Effect.ALLOW,
        actions: ["bedrock:Retrieve"],
        resources: [this.knowledgeBase.knowledgeBaseArn],
      }),
    );

    // Create the Bedrock Agent
    this.agent = new bedrock.CfnAgent(this, "BedrockAgent", {
      agentName: `pdf-agent-${stage}`,
      agentResourceRoleArn: agentRole.roleArn,
      foundationModel: "apac.amazon.nova-pro-v1:0",
      description: "Agent connected to PDF Knowledge Base with Nova Pro",
      instruction: `You are a warm, helpful, and intelligent AI companion designed to assist users with their PDF documents and engage in meaningful conversations.

CRITICAL DOCUMENT ACCESS RESTRICTION:
When a "File URL" is provided in the user's input:
1. FIRST: Try to retrieve and use ONLY data from that exact file URL
2. If the URL retrieval fails or returns no results: Use the document name as a fallback filter to find the file
3. NEVER access or reference any other files in the Knowledge Base besides the one specified (you can respond from your general knowledge though, not the KB)
4. If you cannot find the specified file by URL or name, clearly state that you cannot access the requested document
5. This is a security requirement - users should only see content from files they have permission to access

IMPORTANT: Not every message requires a document search. Respond naturally to greetings, casual chat, and general questions without forcing a document lookup. Only search documents when the user specifically asks about document content or when it would genuinely help answer their question.

Core Behaviors:
- Respond naturally to greetings like "hello", "hi", or "hey" with warmth and friendliness (do not drop summaries for every greeting, just a short introduction)
- Be conversational and personable while maintaining professionalism
- Show genuine interest in helping users understand their documents when they ask about them
- Remember context from our conversation history and reference it naturally
- Engage in back-and-forth dialogue - ask clarifying questions when needed
- Acknowledge when users refer to previous exchanges in the conversation

Document Assistance:
- When users ask about documents, ONLY search the specific file indicated by the File URL or document name
- Provide clear, detailed explanations with specific citations from the PDF
- If information isn't in the specified document, say so clearly and honestly, then answer with what you know (NOT from other sources in the KB)
- Never suggest or reference content from other documents in the Knowledge Base
- Offer to help find related information within the SAME document
- Break down complex document content into digestible explanations

Beyond Documents:
- You can discuss ethical questions, general knowledge, and topics outside the PDFs
- When answering non-document questions, be clear that you're speaking from general knowledge
- Feel free to explore ideas, provide analysis, and engage intellectually
- Help users think through problems even if they're not directly in the documents

Conversation Style:
- Use "I", "you", and "we" naturally - be personable
- Vary your sentence structure and tone to feel human
- Show enthusiasm when appropriate
- Express uncertainty honestly rather than guessing
- Build on the conversation naturally rather than treating each question as isolated
- If a user seems confused or frustrated, adjust your approach to be more helpful

Remember: You're not just answering questions - you're being a thoughtful companion who helps users navigate information and ideas. But you must ALWAYS respect file access boundaries as a non-negotiable security requirement.`,
      idleSessionTtlInSeconds: 600,
      memoryConfiguration: {
        enabledMemoryTypes: ["SESSION_SUMMARY"],
        storageDays: 30,
        sessionSummaryConfiguration: {
          maxRecentSessions: 20,
        },
      },
      knowledgeBases: [
        {
          knowledgeBaseId: this.knowledgeBase.knowledgeBaseId,
          description: "PDF documents knowledge base",
          knowledgeBaseState: "ENABLED",
        },
      ],
    });

    this.agent.node.addDependency(this.knowledgeBase);

    // Create agent alias for production use
    this.agentAlias = new bedrock.CfnAgentAlias(this, "BedrockAgentAlias", {
      agentId: this.agent.attrAgentId,
      agentAliasName: "production",
      description: "Production alias for the PDF agent",
    });

    this.agentAlias.addDependency(this.agent);

    // ========================================
    // KB Sync Lambda Resources
    // ========================================

    const dataBucket = s3.Bucket.fromBucketName(
      this,
      "DataBucket",
      dataSourceBucketName,
    );

    this.syncLambda = new lambda.Function(this, "KbSyncTrigger", {
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
        KNOWLEDGE_BASE_ID: this.knowledgeBase.knowledgeBaseId,
        DATA_SOURCE_ID: this.dataSource.attrDataSourceId,
        DATA_PREFIX: dataSourcePrefix || "",
      },
      timeout: cdk.Duration.seconds(30),
      description: `Triggers KB ingestion on S3 uploads for ${this.knowledgeBase.knowledgeBaseId}`,
    });

    this.syncLambda.node.addDependency(this.dataSource);

    // Grant Lambda permission to start ingestion jobs
    this.syncLambda.addToRolePolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["bedrock:StartIngestionJob"],
        resources: [
          `arn:aws:bedrock:${this.region}:${this.account}:knowledge-base/${this.knowledgeBase.knowledgeBaseId}`,
        ],
      }),
    );

    dataBucket.grantRead(this.syncLambda);

    // Create EventBridge rule to capture S3 object created events
    const rule = new events.Rule(this, "S3ObjectCreatedRule", {
      eventPattern: {
        source: ["aws.s3"],
        detailType: ["Object Created"],
        detail: {
          bucket: {
            name: [dataSourceBucketName],
          },
          ...(dataSourcePrefix && {
            object: {
              key: [{ prefix: dataSourcePrefix }],
            },
          }),
        },
      },
      description: `Triggers KB sync when objects are created in s3://${dataSourceBucketName}${dataSourcePrefix ? "/" + dataSourcePrefix : ""}`,
    });

    rule.addTarget(new targets.LambdaFunction(this.syncLambda));

    // ========================================
    // Outputs
    // ========================================

    new cdk.CfnOutput(this, "KnowledgeBaseId", {
      value: this.knowledgeBase.knowledgeBaseId,
      description: "The ID of the Knowledge Base",
    });

    new cdk.CfnOutput(this, "KnowledgeBaseArn", {
      value: this.knowledgeBase.knowledgeBaseArn,
      description: "The ARN of the Knowledge Base",
    });

    new cdk.CfnOutput(this, "DataSourceId", {
      value: this.dataSource.attrDataSourceId,
      description: "The ID of the Data Source",
    });

    new cdk.CfnOutput(this, "VectorBucketArn", {
      value: vectorBucket.vectorBucketArn,
      description: "The ARN of the S3 Vectors bucket",
    });

    new cdk.CfnOutput(this, "IndexArn", {
      value: vectorIndex.indexArn,
      description: "The ARN of the S3 Vectors index",
    });

    new cdk.CfnOutput(this, "AgentId", {
      value: this.agent.attrAgentId,
      description: "Bedrock Agent ID",
    });

    new cdk.CfnOutput(this, "AgentArn", {
      value: this.agent.attrAgentArn,
      description: "Bedrock Agent ARN",
    });

    new cdk.CfnOutput(this, "AgentAliasId", {
      value: this.agentAlias.attrAgentAliasId,
      description: "Bedrock Agent Alias ID",
    });

    new cdk.CfnOutput(this, "AgentAliasArn", {
      value: this.agentAlias.attrAgentAliasArn,
      description: "Bedrock Agent Alias ARN",
    });

    new cdk.CfnOutput(this, "SyncLambdaArn", {
      value: this.syncLambda.functionArn,
      description: "Lambda function that triggers KB ingestion on S3 uploads",
    });

    new cdk.CfnOutput(this, "SyncLambdaName", {
      value: this.syncLambda.functionName,
      description: "Lambda function name",
    });
  }
}
