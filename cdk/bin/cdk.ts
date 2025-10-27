import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { ImageWorkerStack } from "../lib/image-worker-stack";
import { StorybookStack } from "../lib/storybook-stack";

const app = new App();

const stackName = process.env.STACK;

const env = process.env.ENV || "dev";
// Common host
const host = process.env.DOMAIN || "dev.ratel.foundation";
const webDomain = host;
const apiDomain = `api.${host}`;
const baseDomain = "ratel.foundation";
const deployStorybook =
  (process.env.DEPLOY_STORYBOOK && process.env.DEPLOY_STORYBOOK === "true") ||
  false;

new ImageWorkerStack(app, `ratel-${env}-image-worker`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
});

if (deployStorybook) {
  new StorybookStack(app, `ratel-${env}-storybook`, {
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT,
      region: "us-east-1",
    },
    stage: env,
    commit: process.env.COMMIT!,
    webDomain: `storybook.${host}`,
    baseDomain,
  });
}

new GlobalTableStack(app, `ratel-${env}-dynamodb`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
});

new RegionalServiceStack(app, `ratel-${env}-svc-ap-northeast-2`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
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
