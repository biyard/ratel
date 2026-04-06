import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { DynamoStreamEventStack } from "../lib/dynamo-stream-event";
import { AlbStack } from "../lib/alb-stack";
import { QdrantStack } from "../lib/qdrant-stack";
import { RegionalLambdaStack } from "../lib/regional-lambda-stack";
import { EcsClusterStack } from "../lib/ecs-cluster-stack";
import { VpcEndpointStack } from "../lib/vpc-endpoint-stack";

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

// Shared VPC endpoints (singleton, not stage-scoped — VPC endpoints can only
// exist once per VPC, so dev and prod share this stack).
const vpcEndpointStack = new VpcEndpointStack(
  app,
  `ratel-vpc-endpoints-ap-northeast-2`,
  {
    env: {
      account: awsAccount,
      region: "ap-northeast-2",
    },
  },
);

// Per-stage ECS cluster + CloudMap namespace.
const escStack = new EcsClusterStack(
  app,
  `ratel-${env}-cluster-ap-northeast-2`,
  {
    env: {
      account: awsAccount,
      region: "ap-northeast-2",
    },
    stage: env,
  },
);
escStack.addDependency(vpcEndpointStack);

// High traffic region ECS stack
new RegionalServiceStack(app, `ratel-${env}-svc-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  stage: env,
  commit: process.env.COMMIT!,
  baseDomain,
  apiDomain,
  vpc: escStack.vpc,
  cluster: escStack.cluster,
  namespace: escStack.namespace,
  sharedSecurityGroup: vpcEndpointStack.sharedSecurityGroup,
  icUrl: process.env.IC_URL,
  ratelCanisterId: process.env.RATEL_CANISTER_ID,
  icpIdentityPem: process.env.ICP_IDENTITY_PEM,
});

// Shared ALB for Qdrant gRPC across dev and prod
// const albStack = new AlbStack(app, "ratel-alb-ap-northeast-2", {
//   env: {
//     account: awsAccount,
//     region: "ap-northeast-2",
//   },
//   baseDomain,
//   devDomain: "dev.ratel.foundation",
//   prodDomain: "ratel.foundation",
// });

new QdrantStack(app, `ratel-${env}-qdrant-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  stage: env,
  vpc: escStack.vpc,
  cluster: escStack.cluster,
  namespace: escStack.namespace,
  sharedSecurityGroup: vpcEndpointStack.sharedSecurityGroup,
});

const ap_northeast_2_lambda = new RegionalLambdaStack(
  app,
  `ratel-${env}-lambda-ap-northeast-2`,
  {
    env: {
      account: awsAccount,
      region: "ap-northeast-2",
    },
    stage: env,
    commit: process.env.COMMIT!,
    baseDomain,
    apiDomain: `lambda-${apiDomain}`,
    // Place Lambda in the same VPC as Qdrant so CloudMap private DNS resolves.
    vpc: escStack.vpc,
    sharedSecurityGroup: vpcEndpointStack.sharedSecurityGroup,
  },
);

new RegionalLambdaStack(app, `ratel-${env}-svc-eu-central-1`, {
  env: {
    account: awsAccount,
    region: "eu-central-1",
  },
  stage: env,
  commit: process.env.COMMIT!,
  baseDomain,
  apiDomain,
});

new RegionalLambdaStack(app, `ratel-${env}-svc-us-east-1`, {
  env: {
    account: awsAccount,
    region: "us-east-1",
  },
  stage: env,
  commit: process.env.COMMIT!,
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

new DynamoStreamEventStack(app, `ratel-${env}-stream-ap-northeast-2`, {
  env: {
    account: awsAccount,
    region: "ap-northeast-2",
  },
  stage: env,
  lambdaFunction: ap_northeast_2_lambda.lambdaFunction,
});
