import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";

const app = new App();

const stackName = process.env.STACK;

const env = process.env.ENV || "dev";
// Common host
const host = "dev.ratel.foundation";

// --- Regional stacks (ALB + Fargate) ---
const kr = new RegionalServiceStack(app, `ratel-${env}-svc-ap-northeast-2`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
  enableDaemon: true,
});

const eu = new RegionalServiceStack(app, `ratel-${env}-svc-eu-central-1`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "eu-central-1",
  },
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
});

const us = new RegionalServiceStack(app, `ratel-${env}-svc-us-east-1`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
});

// --- Global Accelerator + Route53 stack ---
// crossRegionReferences=true in all stacks lets us pass ALBs between regions
new GlobalAccelStack(app, "GlobalAccel", {
  stackName,
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  fullDomainName: host,
  euAlb: eu.alb,
  usAlb: us.alb,
  krAlb: kr.alb,
});
