import * as cdk from "aws-cdk-lib";
import * as bedrock from "aws-cdk-lib/aws-bedrock";
import * as s3 from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";

import { KbSyncLambda } from "./constructs/kb-sync-lambda";
import { BedrockAgent } from "./constructs/bedrock-agent";
import { aws_s3vectors } from "aws-cdk-lib";
import * as iam from "aws-cdk-lib/aws-iam";

export interface AiStackProps extends cdk.StackProps {
  /**
   * Deployment stage (e.g., "dev", "prod")
   */
  stage: string;

  agentName: string;
  embeddingModelArn: string;
  foundationModel: string;

  // S3 Bucket
  dataSourceBucketArn: string;
  dataSourceBucketName: string;
  dataSourcePrefix: string;
  dataSourceName: string;

  // Agent
  agentInstruction: string;

  /**
   * Description of the Knowledge Base (optional)
   * @default "Knowledge Base for {knowledgeBaseName}"
   */
  description?: string;
}

export class AiStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: AiStackProps) {
    super(scope, id, props);

    const { stage, embeddingModelArn } = props;
    const baseName = `${props.agentName.toLowerCase().replace(/[^a-z0-9-]/g, "-")}-${stage}`;

    const roleName = `${baseName}-role`;

    const knowledgeBaseName = `${baseName}-knowledge-base`;

    const role = new iam.Role(this, "BedrockKnowledgeBaseRole", {
      roleName,
      assumedBy: new iam.ServicePrincipal("bedrock.amazonaws.com"),
      description: `IAM role for Bedrock Knowledge Base ${knowledgeBaseName}`,
    });

    role.addToPolicy(
      new iam.PolicyStatement({
        actions: ["kms:Decrypt", "kms:GenerateDataKey", "kms:DescribeKey"],
        resources: ["*"],
      }),
    );
    role.addToPolicy(
      new iam.PolicyStatement({
        actions: ["bedrock:InvokeModel"],
        resources: [embeddingModelArn],
      }),
    );

    // ========================================
    // S3 Vectors Resources
    // ========================================
    const vectorBucketName = `${knowledgeBaseName}-vectors`;
    const vectorBucket = new aws_s3vectors.CfnVectorBucket(
      this,
      "VectorBucket",
      {
        vectorBucketName: vectorBucketName,
      },
    );

    const vectorIndexName = `${vectorBucketName}-index`;

    const vectorIndex = new aws_s3vectors.CfnIndex(this, "VectorIndex", {
      vectorBucketName: vectorBucket.vectorBucketName,
      indexName: vectorIndexName,
      dataType: "float32",
      dimension: 1024,
      distanceMetric: "cosine",
      metadataConfiguration: {
        nonFilterableMetadataKeys: [
          "AMAZON_BEDROCK_TEXT",
          "AMAZON_BEDROCK_METADATA",
        ],
      },
    });

    vectorIndex.node.addDependency(vectorBucket);

    role.addToPolicy(
      new iam.PolicyStatement({
        actions: ["s3vectors:*"],
        resources: [
          vectorBucket.attrVectorBucketArn,
          vectorIndex.attrIndexArn,
          `${vectorIndex.attrIndexArn}/*`,
          `${vectorBucket.attrVectorBucketArn}/*`,
        ],
      }),
    );

    // ========================================
    // Knowledge Base
    // ========================================
    const knowledgeBase = new bedrock.CfnKnowledgeBase(this, "KnowledgeBase", {
      name: knowledgeBaseName,
      description: props.description,
      storageConfiguration: {
        s3VectorsConfiguration: {
          indexArn: vectorIndex.attrIndexArn,
          vectorBucketArn: vectorBucket.attrVectorBucketArn,
        },
        type: "S3_VECTORS",
      },
      knowledgeBaseConfiguration: {
        vectorKnowledgeBaseConfiguration: {
          embeddingModelArn,
          embeddingModelConfiguration: {
            bedrockEmbeddingModelConfiguration: {
              dimensions: 1024,
              embeddingDataType: "FLOAT32",
            },
          },
        },
        type: "VECTOR",
      },
      roleArn: role.roleArn,
    });

    knowledgeBase.node.addDependency(vectorBucket);
    knowledgeBase.node.addDependency(vectorIndex);
    knowledgeBase.node.addDependency(role);

    // ========================================
    // Data Source
    // ========================================
    // Original Data bucket
    const dataSourceBucket = s3.Bucket.fromBucketArn(
      this,
      "DataSourceBucket",
      props.dataSourceBucketArn,
    );

    dataSourceBucket.grantRead(role);

    const dataSource = new bedrock.CfnDataSource(this, "DataSource", {
      name: props.dataSourceName,
      description: `S3 data source for ${knowledgeBaseName}`,
      knowledgeBaseId: knowledgeBase.attrKnowledgeBaseId,
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
    dataSource.node.addDependency(dataSourceBucket);
    dataSource.node.addDependency(knowledgeBase);

    // ========================================
    // KB Sync Lambda
    // ========================================
    const syncLambda = new KbSyncLambda(this, "KbSyncLambda", {
      knowledgeBaseId: knowledgeBase.attrKnowledgeBaseId,
      knowledgeBaseArn: knowledgeBase.attrKnowledgeBaseArn,
      dataSourceId: dataSource.attrDataSourceId,
      dataSourceBucketName: props.dataSourceBucketName,
      dataSourcePrefix: props.dataSourcePrefix,
    });

    syncLambda.node.addDependency(dataSource);

    // ========================================
    // Bedrock Agent
    // ========================================
    const bedrockAgent = new BedrockAgent(this, "BedrockAgent", {
      agentName: `${baseName}-agent`,
      knowledgeBaseArn: knowledgeBase.attrKnowledgeBaseArn,
      knowledgeBaseId: knowledgeBase.attrKnowledgeBaseId,
      foundationModel: props.foundationModel,
      description: "Agent connected to PDF Knowledge Base with Nova Pro",
      instruction: props.agentInstruction,
      idleSessionTtlInSeconds: 600,
      memoryConfiguration: {
        enabledMemoryTypes: ["SESSION_SUMMARY"],
        storageDays: 30,
        sessionSummaryConfiguration: {
          maxRecentSessions: 20,
        },
      },
      aliasName: "production",
    });

    bedrockAgent.node.addDependency(knowledgeBase);

    // ========================================
    // Outputs
    // ========================================
    new cdk.CfnOutput(this, "AgentId", {
      value: bedrockAgent.agentId,
      description: "Bedrock Agent ID",
    });

    new cdk.CfnOutput(this, "AgentArn", {
      value: bedrockAgent.agentArn,
      description: "Bedrock Agent ARN",
    });

    new cdk.CfnOutput(this, "AgentAliasId", {
      value: bedrockAgent.agentAliasId,
      description: "Bedrock Agent Alias ID",
    });
  }
}
