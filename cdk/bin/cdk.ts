import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { ImageWorkerStack } from "../lib/image-worker-stack";
import { StaticStack } from "../lib/static-stack";
import { DaemonStack } from "../lib/daemon-stack";
import { KnowledgeBaseStack } from "../lib/knowledge-base-stack";
import { AgentStack } from "../lib/agent-stack";
import { KbSyncStack } from "../lib/kb-sync-stack";

const app = new App();

const stackName = process.env.STACK;

const env = process.env.ENV || "dev";
// Common host
const host = process.env.DOMAIN || "dev.ratel.foundation";
const webDomain = host;
const apiDomain = `api.${host}`;
const baseDomain = "ratel.foundation";

// new ImageWorkerStack(app, `ratel-${env}-image-worker`, {
//   env: {
//     account: process.env.CDK_DEFAULT_ACCOUNT,
//     region: "ap-northeast-2",
//   },
// });

if (env === "dev") {
  new StaticStack(app, `ratel-${env}-storybook`, {
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT,
      region: "us-east-1",
    },
    webDomain: `storybook.${host}`,
    baseDomain,
  });

  new StaticStack(app, `ratel-${env}-report`, {
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT,
      region: "us-east-1",
    },
    webDomain: `report.${host}`,
    baseDomain,
  });
}

new DaemonStack(app, `ratel-${env}-daemon-ap-northeast-2`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  commit: process.env.COMMIT!,
});

new RegionalServiceStack(app, `ratel-${env}-svc-ap-northeast-2`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stage: env,
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
  pghost: process.env.PGHOST_AP!,
  enableDaemon: true,
  baseDomain,
  apiDomain,
});

new RegionalServiceStack(app, `ratel-${env}-svc-eu-central-1`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "eu-central-1",
  },
  stage: env,
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
  pghost: process.env.PGHOST_EU!,
  baseDomain,
  apiDomain,
});

new RegionalServiceStack(app, `ratel-${env}-svc-us-east-1`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  stage: env,
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
  pghost: process.env.PGHOST_US!,
  baseDomain,
  apiDomain,
});

new GlobalAccelStack(app, "GlobalAccel", {
  stackName,
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  stage: env,
  commit: process.env.COMMIT!,

  webDomain,
  apiDomain,
  baseDomain,
});

new GlobalTableStack(app, `ratel-${env}-dynamodb`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
});

// Knowledge Base for PDF AI Helper with S3 Vectors storage
const pdfKnowledgeBaseStack = new KnowledgeBaseStack(app, `ratel-${env}-pdf-knowledge-base`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  knowledgeBaseName: "pdf-KB",
  description: "The knowledge base for the PDF ai helper.",
  dataSourceBucketArn: "arn:aws:s3:::metadata.ratel.foundation",
  dataSourcePrefix: "metadata/",
  dataSourceName: "metadata-s3",
});

// Deploy Bedrock Agent connected to the PDF Knowledge Base
new AgentStack(app, `ratel-${env}-pdf-agent`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stage: env,
  knowledgeBaseId: pdfKnowledgeBaseStack.knowledgeBase.knowledgeBaseId,
  knowledgeBaseArn: pdfKnowledgeBaseStack.knowledgeBase.knowledgeBaseArn,
});

// Deploy KB sync Lambda that triggers ingestion on S3 uploads
new KbSyncStack(app, `ratel-${env}-pdf-kb-sync`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stage: env,
  knowledgeBaseId: pdfKnowledgeBaseStack.knowledgeBase.knowledgeBaseId,
  dataSourceId: pdfKnowledgeBaseStack.dataSource.attrDataSourceId,
  dataBucketName: "metadata.ratel.foundation",
  dataPrefix: "metadata/",
});
