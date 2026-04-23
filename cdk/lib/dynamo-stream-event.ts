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

    // ── Pipe 8: SpaceStatusChangeEvent Insert → SpaceStatusChangeEvent ──
    // Triggers when a new SpaceStatusChangeEvent row is inserted by update_space
    new pipes.CfnPipe(this, "SpaceStatusChangeEventPipe", {
      name: `ratel-${stage}-space-status-change-event-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_STATUS_CHANGE_EVENT#" }] },
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
          detailType: "SpaceStatusChangeEvent",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route SpaceStatusChangeEvent events to app-shell Lambda ───
    new events.Rule(this, "SpaceStatusChangeEventRule", {
      eventBus,
      description:
        "Route space status change events to app-shell for notification fan-out",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["SpaceStatusChangeEvent"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 9: Poll Answer Insert → PollXpRecord ────────────────────
    // Triggers when a new SpacePollUserAnswer is inserted (user responds to poll)
    new pipes.CfnPipe(this, "PollXpPipe", {
      name: `ratel-${stage}-poll-xp-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POLL_USER_ANSWER#" }] },
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
          detailType: "PollXpRecord",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route PollXpRecord events to app-shell Lambda ──────────
    new events.Rule(this, "PollXpRecordRule", {
      eventBus,
      description:
        "Route poll answer events to app-shell for XP recording",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["PollXpRecord"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 10: Quiz Attempt Insert → QuizXpRecord ──────────────────
    // Triggers when a new SpaceQuizAttempt is inserted (user attempts quiz)
    new pipes.CfnPipe(this, "QuizXpPipe", {
      name: `ratel-${stage}-quiz-xp-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_QUIZ_ATTEMPT#" }] },
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
          detailType: "QuizXpRecord",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route QuizXpRecord events to app-shell Lambda ──────────
    new events.Rule(this, "QuizXpRecordRule", {
      eventBus,
      description:
        "Route quiz attempt events to app-shell for XP recording",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["QuizXpRecord"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 11: Discussion Comment Insert → DiscussionXpRecord ──────
    // Triggers when a new SpacePostComment is inserted (comment on discussion)
    new pipes.CfnPipe(this, "DiscussionCommentXpPipe", {
      name: `ratel-${stage}-discussion-comment-xp-pipe`,
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
          detailType: "DiscussionXpRecord",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Pipe 11b: Discussion Reply Insert → DiscussionXpRecord ───────
    // Triggers when a new SpacePostCommentReply is inserted (reply to comment)
    new pipes.CfnPipe(this, "DiscussionReplyXpPipe", {
      name: `ratel-${stage}-discussion-reply-xp-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POST_COMMENT_REPLY#" }] },
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
          detailType: "DiscussionXpRecord",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route DiscussionXpRecord events to app-shell Lambda ────
    new events.Rule(this, "DiscussionXpRecordRule", {
      eventBus,
      description:
        "Route discussion comment/reply events to app-shell for XP recording",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["DiscussionXpRecord"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe 12: Follow Insert → FollowXpRecord ─────────────────────
    // Triggers when a new Follower record is inserted (user follows someone)
    new pipes.CfnPipe(this, "FollowXpPipe", {
      name: `ratel-${stage}-follow-xp-pipe`,
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
                    sk: { S: [{ prefix: "FOLLOWER#" }] },
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
          detailType: "FollowXpRecord",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route FollowXpRecord events to app-shell Lambda ────────
    new events.Rule(this, "FollowXpRecordRule", {
      eventBus,
      description:
        "Route follow events to app-shell for XP recording",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["FollowXpRecord"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ─────────────────────────────────────────────────────────────────
    // Essence indexing — moved off the synchronous controller path so
    // creates/updates don't pay an extra DynamoDB roundtrip on the hot
    // path. Each entity type gets two pipes (INSERT/MODIFY → IndexX,
    // REMOVE → DeleteX). The Lambda dispatch lives in
    // `EventBridgeEnvelope::proc` and shares its handler with the
    // migrate endpoint via `essence::services`.
    // ─────────────────────────────────────────────────────────────────

    // ── Pipe: Post insert/modify → EssenceIndexPost ─────────────────
    new pipes.CfnPipe(this, "EssenceIndexPostPipe", {
      name: `ratel-${stage}-essence-index-post-pipe`,
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
                    sk: { S: ["POST"] },
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
          detailType: "EssenceIndexPost",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    new events.Rule(this, "EssenceIndexPostRule", {
      eventBus,
      description: "Route post insert/update to app-shell for essence indexing",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceIndexPost"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: Post remove → EssenceDeletePost ───────────────────────
    new pipes.CfnPipe(this, "EssenceDeletePostPipe", {
      name: `ratel-${stage}-essence-delete-post-pipe`,
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
                    sk: { S: ["POST"] },
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
          detailType: "EssenceDeletePost",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    new events.Rule(this, "EssenceDeletePostRule", {
      eventBus,
      description: "Route post removes to app-shell for essence detach",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceDeletePost"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: PostComment insert/modify → EssenceIndexPostComment ──
    // Top-level post comments. Replies are routed through a separate pipe
    // (below) to keep each filter to a single well-established prefix match.
    new pipes.CfnPipe(this, "EssenceIndexPostCommentPipe", {
      name: `ratel-${stage}-essence-index-post-comment-pipe`,
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
                    sk: { S: [{ prefix: "POST_COMMENT#" }] },
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
          detailType: "EssenceIndexPostComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Pipe: PostCommentReply insert/modify → EssenceIndexPostComment ──
    // Reply entities route to the SAME detailType so one Lambda handler
    // covers both (they share the `PostComment` Rust model). The existing
    // Rule already routes `EssenceIndexPostComment` — this pipe just adds
    // another source flow into it.
    new pipes.CfnPipe(this, "EssenceIndexPostCommentReplyPipe", {
      name: `ratel-${stage}-essence-index-post-comment-reply-pipe`,
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
                    sk: { S: [{ prefix: "POST_COMMENT_REPLY#" }] },
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
          detailType: "EssenceIndexPostComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    new events.Rule(this, "EssenceIndexPostCommentRule", {
      eventBus,
      description:
        "Route post comment insert/update to app-shell for essence indexing",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceIndexPostComment"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: PostComment remove → EssenceDeletePostComment ─────────
    new pipes.CfnPipe(this, "EssenceDeletePostCommentPipe", {
      name: `ratel-${stage}-essence-delete-post-comment-pipe`,
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
                    sk: { S: [{ prefix: "POST_COMMENT#" }] },
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
          detailType: "EssenceDeletePostComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    // ── Pipe: PostCommentReply remove → EssenceDeletePostComment ────
    new pipes.CfnPipe(this, "EssenceDeletePostCommentReplyPipe", {
      name: `ratel-${stage}-essence-delete-post-comment-reply-pipe`,
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
                    sk: { S: [{ prefix: "POST_COMMENT_REPLY#" }] },
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
          detailType: "EssenceDeletePostComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    new events.Rule(this, "EssenceDeletePostCommentRule", {
      eventBus,
      description: "Route post comment removes to app-shell for essence detach",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceDeletePostComment"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: SpacePostComment insert/modify → EssenceIndexDiscussionComment ──
    // Top-level discussion comments. Replies go through a separate pipe
    // below. `SPACE_POST_COMMENT_LIKE#` is excluded by the `#` boundary.
    new pipes.CfnPipe(this, "EssenceIndexDiscussionCommentPipe", {
      name: `ratel-${stage}-essence-index-discussion-comment-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POST_COMMENT#" }] },
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
          detailType: "EssenceIndexDiscussionComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Pipe: SpacePostCommentReply insert/modify → EssenceIndexDiscussionComment ──
    // Reply entities route to the SAME detailType as top-level comments
    // (shared `SpacePostComment` Rust model).
    new pipes.CfnPipe(this, "EssenceIndexDiscussionCommentReplyPipe", {
      name: `ratel-${stage}-essence-index-discussion-comment-reply-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POST_COMMENT_REPLY#" }] },
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
          detailType: "EssenceIndexDiscussionComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    new events.Rule(this, "EssenceIndexDiscussionCommentRule", {
      eventBus,
      description:
        "Route discussion comment + reply insert/update to app-shell for essence indexing",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceIndexDiscussionComment"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: SpacePostComment remove → EssenceDeleteDiscussionComment ──
    new pipes.CfnPipe(this, "EssenceDeleteDiscussionCommentPipe", {
      name: `ratel-${stage}-essence-delete-discussion-comment-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POST_COMMENT#" }] },
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
          detailType: "EssenceDeleteDiscussionComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    // ── Pipe: SpacePostCommentReply remove → EssenceDeleteDiscussionComment ──
    new pipes.CfnPipe(this, "EssenceDeleteDiscussionCommentReplyPipe", {
      name: `ratel-${stage}-essence-delete-discussion-comment-reply-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POST_COMMENT_REPLY#" }] },
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
          detailType: "EssenceDeleteDiscussionComment",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    new events.Rule(this, "EssenceDeleteDiscussionCommentRule", {
      eventBus,
      description:
        "Route discussion comment + reply removes to app-shell for essence detach",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceDeleteDiscussionComment"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: SpacePoll insert/modify → EssenceIndexPoll ────────────
    new pipes.CfnPipe(this, "EssenceIndexPollPipe", {
      name: `ratel-${stage}-essence-index-poll-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POLL#" }] },
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
          detailType: "EssenceIndexPoll",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    new events.Rule(this, "EssenceIndexPollRule", {
      eventBus,
      description:
        "Route space poll insert/update to app-shell for essence indexing",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceIndexPoll"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: SpacePoll remove → EssenceDeletePoll ──────────────────
    new pipes.CfnPipe(this, "EssenceDeletePollPipe", {
      name: `ratel-${stage}-essence-delete-poll-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_POLL#" }] },
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
          detailType: "EssenceDeletePoll",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    new events.Rule(this, "EssenceDeletePollRule", {
      eventBus,
      description: "Route space poll removes to app-shell for essence detach",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceDeletePoll"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: SpaceQuiz insert/modify → EssenceIndexQuiz ────────────
    new pipes.CfnPipe(this, "EssenceIndexQuizPipe", {
      name: `ratel-${stage}-essence-index-quiz-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_QUIZ#" }] },
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
          detailType: "EssenceIndexQuiz",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    new events.Rule(this, "EssenceIndexQuizRule", {
      eventBus,
      description:
        "Route space quiz insert/update to app-shell for essence indexing",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceIndexQuiz"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: SpaceQuiz remove → EssenceDeleteQuiz ──────────────────
    new pipes.CfnPipe(this, "EssenceDeleteQuizPipe", {
      name: `ratel-${stage}-essence-delete-quiz-pipe`,
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
                    sk: { S: [{ prefix: "SPACE_QUIZ#" }] },
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
          detailType: "EssenceDeleteQuiz",
        },
        inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
      },
    });

    new events.Rule(this, "EssenceDeleteQuizRule", {
      eventBus,
      description: "Route space quiz removes to app-shell for essence detach",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceDeleteQuiz"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });

    // ── Pipe: SpaceAction insert/modify → EssenceActionMetadataUpdate ──
    // Quiz essence rows derive title/description from SpaceAction. When the
    // action metadata changes (admin fills in the initially-empty title/desc
    // via `update_space_action`), re-index the related SpaceQuiz so the
    // essence row picks up the new copy.
    new pipes.CfnPipe(this, "EssenceActionMetadataPipe", {
      name: `ratel-${stage}-essence-action-metadata-pipe`,
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
                    sk: { S: ["SPACE_ACTION"] },
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
          detailType: "EssenceActionMetadataUpdate",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    new events.Rule(this, "EssenceActionMetadataRule", {
      eventBus,
      description:
        "Route space action metadata updates to app-shell for quiz essence re-index",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["EssenceActionMetadataUpdate"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });
  }
}
