import {
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_elasticloadbalancingv2 as elbv2,
  aws_iam as iam,
  Stack,
  StackProps,
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";

import * as lambda from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import * as sqs from "aws-cdk-lib/aws-sqs";
import * as lambdaEventSources from "aws-cdk-lib/aws-lambda-event-sources";

export interface ImageWorkerStackProps extends StackProps {}

export class ImageWorkerStack extends Stack {
  constructor(scope: Construct, id: string, props: ImageWorkerStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });
    const codePath = ".build/image-worker";
    const func = new lambda.Function(this, "Function", {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      code: lambda.Code.fromAsset(codePath),
      handler: "bootstrap",
      environment: {
        NO_COLOR: "true",
      },
      memorySize: 512,
      timeout: cdk.Duration.seconds(30),
    });

    const queue = new sqs.Queue(this, "Queue", {
      visibilityTimeout: cdk.Duration.seconds(300),
      retentionPeriod: cdk.Duration.days(14),
      fifo: false,
    });

    func.addEventSource(
      new lambdaEventSources.SqsEventSource(queue, {
        batchSize: 10,
        maxBatchingWindow: cdk.Duration.seconds(5),
        reportBatchItemFailures: true,
      }),
    );

    queue.grantConsumeMessages(func);
  }
}
