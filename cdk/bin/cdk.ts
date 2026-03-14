import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { StaticStack } from "../lib/static-stack";
import { DaemonStack } from "../lib/daemon-stack";
import { QdrantStack } from "../lib/qdrant-stack";
import { RegionalLambdaStack } from "../lib/regional-lambda-stack";
import { EcsClusterStack } from "../lib/ecs-cluster-stack";

const app = new App();

const stackName = process.env.STACK;
const awsAccount =
  process.env.CDK_DEFAULT_ACCOUNT || process.env.AWS_ACCOUNT_ID;

const env = process.env.ENV || "dev";
const highTrafficRegions = (process.env.HIGH_TRAFFIC_REGION || "")
  .split(",")
  .map((r) => r.trim())
  .filter((r) => r.length > 0);
// Common host
const host = process.env.DOMAIN || "dev.ratel.foundation";
const webDomain = host;
const apiDomain = `api.${host}`;
const baseDomain = "ratel.foundation";

const escStack = new EcsClusterStack(
  app,
  `ratel-${env}-cluster-ap-northeast-2`,
  {
    env: {
      account: awsAccount,
      region: "ap-northeast-2",
    },
  },
);

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
  enableEcs: highTrafficRegions.includes("ap-northeast-2"),
  vpc: escStack.vpc,
  cluster: escStack.cluster,
  namespace: escStack.namespace,
});

new DaemonStack(app, `ratel-${env}-daemon-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  commit: process.env.COMMIT!,
  vpc: escStack.vpc,
  cluster: escStack.cluster,
});

new QdrantStack(app, `ratel-${env}-qdrant-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  stage: env,
  vpc: escStack.vpc,
  cluster: escStack.cluster,
  qdrantApiKey: process.env.QDRANT_API_KEY,
  baseDomain,
  vectorDomain: `vector.${host}`,
});

new RegionalLambdaStack(app, `ratel-${env}-lambda-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  stage: env,
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
