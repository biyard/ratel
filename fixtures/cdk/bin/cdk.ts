#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { CdkStack } from "../lib/cdk-stack";

const app = new cdk.App();

new CdkStack(app, `AppStack`, {
  stackName: `${process.env.PROJECT}-${process.env.SERVICE}-${process.env.ENV}-stack`,

  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
});
