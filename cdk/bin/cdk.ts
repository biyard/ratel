import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { ImageWorkerStack } from "../lib/image-worker-stack";
import { StaticStack } from "../lib/static-stack";
import { DaemonStack } from "../lib/daemon-stack";
import { BedrockAgentStack } from "../lib/bedrock-agent-stack";

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

// Create Bedrock Agent stack (uses manually created KB and Agent from AWS Console)
new BedrockAgentStack(app, `ratel-${env}-bedrock-agent`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2", // Must match S3 bucket region
  },
  stage: env,
  knowledgeBaseId: process.env.BEDROCK_KNOWLEDGE_BASE_ID || "your-kb-id",
  dataSourceId: process.env.BEDROCK_DATA_SOURCE_ID || "your-data-source-id",
  pdfBucketName: process.env.PDF_BUCKET_NAME || "your-bucket-name",
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
