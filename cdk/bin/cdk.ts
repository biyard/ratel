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

// Launchpad point-integration partner config — injected as RUNTIME Lambda env
// (read via `std::env::var` in `features/launchpad_partner/config.rs`, which
// prefers runtime over the compile-time `option_env!` fallback). Sourced from
// CI secrets in the CDK deploy step. Only set keys that are present so an unset
// secret falls back to ratel's built-in default instead of overriding it with "".
const launchpadEnv: { [key: string]: string } = {};
if (process.env.LAUNCHPAD_BASE_URL)
  launchpadEnv.LAUNCHPAD_BASE_URL = process.env.LAUNCHPAD_BASE_URL;
if (process.env.LAUNCHPAD_PROJECT_ID)
  launchpadEnv.LAUNCHPAD_PROJECT_ID = process.env.LAUNCHPAD_PROJECT_ID;
if (process.env.LAUNCHPAD_PARTNER_SECRET)
  launchpadEnv.LAUNCHPAD_PARTNER_SECRET = process.env.LAUNCHPAD_PARTNER_SECRET;
if (process.env.LAUNCHPAD_POINT_SYMBOL)
  launchpadEnv.LAUNCHPAD_POINT_SYMBOL = process.env.LAUNCHPAD_POINT_SYMBOL;

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
// new RegionalServiceStack(app, `ratel-${env}-svc-ap-northeast-2`, {
//   env: {
//     account: awsAccount,
//     region: "ap-northeast-2",
//   },
//   stage: env,
//   commit: process.env.COMMIT!,
//   baseDomain,
//   apiDomain,
//   vpc: escStack.vpc,
//   cluster: escStack.cluster,
//   namespace: escStack.namespace,
//   sharedSecurityGroup: vpcEndpointStack.sharedSecurityGroup,
//   icUrl: process.env.IC_URL,
//   ratelCanisterId: process.env.RATEL_CANISTER_ID,
//   icpIdentityPem: process.env.ICP_IDENTITY_PEM,
// });

const ap_northeast_2_svc = new RegionalLambdaStack(
  app,
  `ratel-${env}-svc-ap-northeast-2`,
  {
    env: {
      account: awsAccount,
      region: "ap-northeast-2",
    },
    stage: env,
    commit: process.env.COMMIT!,
    baseDomain,
    apiDomain,
    webOrigin: `https://${webDomain}`,
    runtimeEnvironment: {
      ...launchpadEnv,
    },
  },
);

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

const qdrantStack = new QdrantStack(app, `ratel-${env}-qdrant-ap-northeast-2`, {
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
    webOrigin: `https://${webDomain}`,
    runtimeEnvironment: {
      ...launchpadEnv,
    },
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
  webOrigin: `https://${webDomain}`,
  runtimeEnvironment: {
    ...launchpadEnv,
  },
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
  webOrigin: `https://${webDomain}`,
  runtimeEnvironment: {
    ...launchpadEnv,
  },
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
  // Route the analyze pipeline rules to the dedicated higher-memory
  // Lambda created alongside the API Lambda in the same regional
  // stack. Falls back transparently when undefined.
  analyzeLambdaFunction: ap_northeast_2_lambda.analyzeLambdaFunction,
  // Cross-posting rules go to the non-VPC svc Lambda because the
  // VPC-attached `lambdaFunction` has no NAT egress and can't reach
  // bsky.social / api.linkedin.com — every dispatch hits the 30 s
  // Lambda timeout and the SyndicationJob row sticks in `pending`
  // with the lock held. Remove this override once NAT is added to
  // the shared VPC.
  crossPostingLambda: ap_northeast_2_svc.lambdaFunction,
  // Push fan-out (FCM) needs internet egress to Google's APIs, which
  // the VPC `lambdaFunction` lacks (no NAT) — the FCM OAuth token
  // request fails and no push is delivered (emails still send via the
  // SES VPC endpoint). Route it to the non-VPC svc Lambda. Remove once
  // NAT is added to the shared VPC.
  pushLambda: ap_northeast_2_svc.lambdaFunction,
});
