import {
  Stack,
  StackProps,
  aws_events as events,
  aws_events_targets as eventsTargets,
  aws_iam as iam,
  aws_lambda as lambda,
  aws_pipes as pipes,
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export interface DynamoStreamEventStackProps extends StackProps {
  stage: string;
  lambdaFunction: lambda.Function;
}

export class DynamoStreamEventStack extends Stack {
  constructor(
    scope: Construct,
    id: string,
    props: DynamoStreamEventStackProps,
  ) {
    super(scope, id, props);

    const { stage } = props;

    // Import the existing EventBridge bus
    const eventBus = events.EventBus.fromEventBusName(
      this,
      "RatelEventBus",
      `ratel-${stage}-bus`,
    );

    // Import the DynamoDB table stream ARN
    const mainTableStreamArn = cdk.Fn.importValue(
      `ratel-${stage}-main-stream-arn`,
    );

    // Shared IAM role for EventBridge Pipes
    const pipeRole = new iam.Role(this, "TimelinePipeRole", {
      roleName: `ratel-${stage}-timeline-pipe-role`,
      assumedBy: new iam.ServicePrincipal("pipes.amazonaws.com"),
    });

    pipeRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: [
          "dynamodb:DescribeStream",
          "dynamodb:GetRecords",
          "dynamodb:GetShardIterator",
          "dynamodb:ListStreams",
        ],
        resources: [mainTableStreamArn],
      }),
    );

    eventBus.grantPutEventsTo(pipeRole);

    // ── Pipe 1: Post Publish → TimelineUpdate ────────────────────────
    // Triggers when a post's status changes to "Published"
    new pipes.CfnPipe(this, "TimelinePipe", {
      name: `ratel-${stage}-timeline-pipe`,
      roleArn: pipeRole.roleArn,
      source: mainTableStreamArn,
      sourceParameters: {
        dynamoDbStreamParameters: {
          startingPosition: "LATEST",
          batchSize: 10,
        },
        filterCriteria: {
          filters: [
            {
              pattern: JSON.stringify({
                eventName: ["MODIFY"],
                dynamodb: {
                  NewImage: {
                    sk: { S: [{ prefix: "POST" }] },
                    status: { S: ["PUBLISHED"] },
                  },
                },
              }),
            },
          ],
        },
      },
      target: eventBus.eventBusArn,
      targetParameters: {
        eventBridgeEventBusParameters: {
          source: "ratel.dynamodb.stream",
          detailType: "TimelineUpdate",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Pipe 2: Engagement Change → PopularPostUpdate ────────────────
    // Triggers when likes/comments/shares change on a published post
    new pipes.CfnPipe(this, "PopularPostPipe", {
      name: `ratel-${stage}-popular-post-pipe`,
      roleArn: pipeRole.roleArn,
      source: mainTableStreamArn,
      sourceParameters: {
        dynamoDbStreamParameters: {
          startingPosition: "LATEST",
          batchSize: 10,
        },
        filterCriteria: {
          filters: [
            {
              pattern: JSON.stringify({
                eventName: ["MODIFY"],
                dynamodb: {
                  NewImage: {
                    sk: { S: [{ prefix: "POST" }] },
                    status: { S: ["PUBLISHED"] },
                    likes: { N: [{ exists: true }] },
                  },
                },
              }),
            },
          ],
        },
      },
      target: eventBus.eventBusArn,
      targetParameters: {
        eventBridgeEventBusParameters: {
          source: "ratel.dynamodb.stream",
          detailType: "PopularPostUpdate",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Pipe 3: Participant Change → PopularSpaceUpdate ──────────────
    // Triggers when participants field changes on a SpaceCommon entity
    new pipes.CfnPipe(this, "PopularSpacePipe", {
      name: `ratel-${stage}-popular-space-pipe`,
      roleArn: pipeRole.roleArn,
      source: mainTableStreamArn,
      sourceParameters: {
        dynamoDbStreamParameters: {
          startingPosition: "LATEST",
          batchSize: 10,
        },
        filterCriteria: {
          filters: [
            {
              pattern: JSON.stringify({
                eventName: ["MODIFY"],
                dynamodb: {
                  NewImage: {
                    sk: { S: ["SPACE_COMMON"] },
                    participants: { N: [{ exists: true }] },
                  },
                },
              }),
            },
          ],
        },
      },
      target: eventBus.eventBusArn,
      targetParameters: {
        eventBridgeEventBusParameters: {
          source: "ratel.dynamodb.stream",
          detailType: "PopularSpaceUpdate",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route TimelineUpdate events to app-shell Lambda ────────
    new events.Rule(this, "TimelineUpdateRule", {
      eventBus,
      description:
        "Route post publish events to app-shell for follower/team fan-out",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["TimelineUpdate"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Rule: Route PopularPostUpdate events to app-shell Lambda ─────
    new events.Rule(this, "PopularPostUpdateRule", {
      eventBus,
      description: "Route popular post events to app-shell for broader fan-out",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["PopularPostUpdate"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Rule: Route PopularSpaceUpdate events to app-shell Lambda ────
    new events.Rule(this, "PopularSpaceUpdateRule", {
      eventBus,
      description:
        "Route popular space events to app-shell for broader fan-out",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["PopularSpaceUpdate"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });
  }
}
