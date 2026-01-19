import * as cdk from "aws-cdk-lib";
import * as bedrock from "aws-cdk-lib/aws-bedrock";
import * as s3 from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";
import * as s3Vectors from "./s3-vectors/bucket";
import * as s3VectorsIndex from "./s3-vectors/index";
import * as s3VectorsKB from "./s3-vectors/knowledge-base";

export interface KnowledgeBaseStackProps extends cdk.StackProps {
  knowledgeBaseName: string;
  description: string;
  dataSourceBucketArn: string;
  dataSourcePrefix: string;
  dataSourceName: string;
}

export class KnowledgeBaseStack extends cdk.Stack {
  public readonly knowledgeBase: s3VectorsKB.KnowledgeBase;
  public readonly dataSource: bedrock.CfnDataSource;

  constructor(scope: Construct, id: string, props: KnowledgeBaseStackProps) {
    super(scope, id, props);

    const kbNameSanitized = props.knowledgeBaseName
      .toLowerCase()
      .replace(/[^a-z0-9-]/g, "-");

    // Create S3 Vectors bucket
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
      // Make large Bedrock metadata keys non-filterable to avoid 2048 byte limit
      // Non-filterable metadata has a 40KB limit instead of 2048 bytes
      metadataConfiguration: {
        nonFilterableMetadataKeys: [
          "AMAZON_BEDROCK_TEXT",
          "AMAZON_BEDROCK_METADATA"
        ],
      },
    });

    // Add dependency
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

    // Add dependencies for knowledge base
    this.knowledgeBase.node.addDependency(vectorIndex);
    this.knowledgeBase.node.addDependency(vectorBucket);

    // Get the data source bucket
    const dataSourceBucket = s3.Bucket.fromBucketArn(
      this,
      "DataSourceBucket",
      props.dataSourceBucketArn
    );

    // Grant read access to the KB role for the data source bucket
    dataSourceBucket.grantRead(this.knowledgeBase.role);

    // Create CUSTOM Data Source (no auto-scanning)
    // Documents are ingested directly via API with specific file lists
    this.dataSource = new bedrock.CfnDataSource(this, "DataSource", {
      name: props.dataSourceName,
      description: `Custom data source for ${props.knowledgeBaseName} - direct ingestion only`,
      knowledgeBaseId: this.knowledgeBase.knowledgeBaseId,
      dataSourceConfiguration: {
        type: "CUSTOM",
      },
    });

    // Still grant S3 read access for the KB to fetch documents during ingestion
    dataSourceBucket.grantRead(this.knowledgeBase.role);

    // Add dependencies for data source
    this.dataSource.node.addDependency(this.knowledgeBase);
    this.dataSource.node.addDependency(dataSourceBucket);

    // Outputs
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
  }
}
