import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { StaticStack } from "../lib/static-stack";
import { DaemonStack } from "../lib/daemon-stack";
import { QdrantStack } from "../lib/qdrant-stack";

const app = new App();

const stackName = process.env.STACK;
const awsAccount =
  process.env.CDK_DEFAULT_ACCOUNT || process.env.AWS_ACCOUNT_ID;

const env = process.env.ENV || "dev";
// Common host
const host = process.env.DOMAIN || "dev.ratel.foundation";
const webDomain = host;
const apiDomain = `api.${host}`;
const baseDomain = "ratel.foundation";

// new ImageWorkerStack(app, `ratel-${env}-image-worker`, {
//   env: {
//     account: awsAccount,
//     region: "ap-northeast-2",
//   },
// });

const daemonStack = new DaemonStack(app, `ratel-${env}-daemon-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  commit: process.env.COMMIT!,
});

new QdrantStack(app, `ratel-${env}-qdrant-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  stage: env,
  cluster: daemonStack.cluster,
  vpc: daemonStack.vpc,
  qdrantApiKey: process.env.QDRANT_API_KEY,
  baseDomain,
  vectorDomain: `vector.${host}`,
});

new RegionalServiceStack(app, `ratel-${env}-svc-ap-northeast-2`, {
  env: {
    account: awsAccount,
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
    account: awsAccount,
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
    account: awsAccount,
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
    account: awsAccount,
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
    account: awsAccount,
    region: "ap-northeast-2",
  },
});
