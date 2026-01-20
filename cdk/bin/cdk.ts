import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { StaticStack } from "../lib/static-stack";
import { DaemonStack } from "../lib/daemon-stack";
import { AiAgentStack } from "../lib/ai-agent-stack";

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

// AI Agent Stack for PDF AI Helper (includes Knowledge Base, Agent, and KB Sync)
new AiAgentStack(app, `ratel-${env}-ai-agent`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stage: env,
  knowledgeBaseName: "pdf-KB",
  description: "The knowledge base for the PDF ai helper.",
  dataSourceBucketArn: "arn:aws:s3:::metadata.ratel.foundation",
  dataSourceBucketName: "metadata.ratel.foundation",
  dataSourcePrefix: "metadata/",
  dataSourceName: "metadata-s3",
});
