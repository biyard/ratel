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

    // ── Pipe 1b: Post Publish → PostVectorIndex ─────────────────────
    // Triggers when a post is published (same filter as TimelinePipe) for Qdrant indexing
    new pipes.CfnPipe(this, "PostVectorIndexPipe", {
      name: `ratel-${stage}-post-vector-index-pipe`,
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
                eventName: ["INSERT", "MODIFY"],
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
          detailType: "PostVectorIndex",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route PostVectorIndex events to app-shell Lambda ─────
    new events.Rule(this, "PostVectorIndexRule", {
      eventBus,
      description:
        "Route post publish events to app-shell for Qdrant vector indexing",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["PostVectorIndex"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 1c: Post Delete → PostVectorDelete ───────────────────
    // Triggers when a published post is removed
    new pipes.CfnPipe(this, "PostVectorDeletePipe", {
      name: `ratel-${stage}-post-vector-delete-pipe`,
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
                eventName: ["REMOVE"],
                dynamodb: {
                  OldImage: {
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
          detailType: "PostVectorDelete",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    // ── Rule: Route PostVectorDelete events to app-shell Lambda ────
    new events.Rule(this, "PostVectorDeleteRule", {
      eventBus,
      description:
        "Route post delete events to app-shell for Qdrant vector removal",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["PostVectorDelete"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
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

    // ── Pipe 4: Discussion Reply → AiModeratorReplyCheck ────────────
    // Triggers when a SpacePost entity's comments field changes (reply added)
    new pipes.CfnPipe(this, "AiModeratorPipe", {
      name: `ratel-${stage}-ai-moderator-pipe`,
      roleArn: pipeRole.roleArn,
      source: mainTableStreamArn,
      sourceParameters: {
        dynamoDbStreamParameters: {
          startingPosition: "LATEST",
          batchSize: 1,
        },
        filterCriteria: {
          filters: [
            {
              pattern: JSON.stringify({
                eventName: ["MODIFY"],
                dynamodb: {
                  NewImage: {
                    sk: { S: [{ prefix: "SPACE_POST#" }] },
                    comments: { N: [{ exists: true }] },
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
          detailType: "AiModeratorReplyCheck",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route AiModeratorReplyCheck events to app-shell Lambda ─
    new events.Rule(this, "AiModeratorReplyCheckRule", {
      eventBus,
      description:
        "Route discussion reply events to app-shell for AI moderator check",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["AiModeratorReplyCheck"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 5: Discussion Comment Insert → AiModeratorReplyIndex ───
    // Triggers when a new SpacePostComment is inserted (reply added to discussion)
    new pipes.CfnPipe(this, "AiModeratorReplyIndexPipe", {
      name: `ratel-${stage}-ai-moderator-reply-index-pipe`,
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
                eventName: ["INSERT"],
                dynamodb: {
                  NewImage: {
                    sk: { S: [{ prefix: "SPACE_POST_COMMENT#" }] },
                    content: { S: [{ exists: true }] },
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
          detailType: "AiModeratorReplyIndex",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route AiModeratorReplyIndex events to app-shell Lambda ─
    new events.Rule(this, "AiModeratorReplyIndexRule", {
      eventBus,
      description:
        "Route new discussion comments to app-shell for Qdrant indexing",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["AiModeratorReplyIndex"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 6: Notification Insert → NotificationSend ─────────────
    // Triggers when a new Notification entity is inserted
    new pipes.CfnPipe(this, "NotificationPipe", {
      name: `ratel-${stage}-notification-pipe`,
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
                eventName: ["INSERT"],
                dynamodb: {
                  NewImage: {
                    pk: { S: [{ prefix: "NOTIFICATION#" }] },
                    sk: { S: [{ prefix: "NOTIFICATION#" }] },
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
          detailType: "NotificationSend",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route NotificationSend events to app-shell Lambda ────
    new events.Rule(this, "NotificationSendRule", {
      eventBus,
      description:
        "Route notification events to app-shell for email/SMS delivery",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["NotificationSend"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 7: Space Activity Insert → ActivityScoreAggregate ──────
    // Triggers when a new SpaceActivity record is inserted (user performs an action)
    new pipes.CfnPipe(this, "ActivityScorePipe", {
      name: `ratel-${stage}-activity-score-pipe`,
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
                eventName: ["INSERT"],
                dynamodb: {
                  NewImage: {
                    sk: { S: [{ prefix: "SPACE_ACTIVITY#" }] },
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
          detailType: "ActivityScoreAggregate",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route ActivityScoreAggregate events to app-shell Lambda ─
    new events.Rule(this, "ActivityScoreAggregateRule", {
      eventBus,
      description:
        "Route space activity events to app-shell for score aggregation",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["ActivityScoreAggregate"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });
  }
}
