import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack.js";
import { GlobalAccelStack } from "../lib/global-accel-stack.js";

const app = new App();

let stackName = `${process.env.PROJECT}-${process.env.SERVICE}-${process.env.ENV}-stack`;
if (process.env.STACK) {
  stackName = process.env.STACK;
}
stackName = "ratel-api2-dev-stack"
// Common host
const host = "api2.dev.ratel.foundation";

// --- Regional stacks (ALB + Fargate) ---
const eu = new RegionalServiceStack(app, "Svc-EU", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "eu-central-1" },
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
});

const us = new RegionalServiceStack(app, "Svc-US", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1" },
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
});

const kr = new RegionalServiceStack(app, "Svc-KR", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2" },
  fullDomainName: host,
  healthCheckPath: "/version",
  commit: process.env.COMMIT!,
});

// --- Global Accelerator + Route53 stack ---
// crossRegionReferences=true in all stacks lets us pass ALBs between regions
new GlobalAccelStack(app, "GlobalAccel", {
  stackName,
  // GA is a global service; pick any region for the stack (us-west-2 is common)
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
  fullDomainName: host,
  euAlb: eu.alb,
  usAlb: us.alb,
  krAlb: kr.alb
});
