---
globs: ["cdk/lib/dynamo-stream-event.ts", "app/ratel/src/common/types/event_bridge_envelope.rs", "app/ratel/src/common/stream_handler.rs"]
---

# Implementing EventBridge Events

When adding a new DynamoDB Stream-driven event, three files must be updated in lockstep.

## Architecture

```
DynamoDB Stream → CDK Pipe (filter) → EventBridge Bus → Rule → Lambda → EventBridgeEnvelope::proc()
                                                                              ↓
                                                               stream_handler (local-dev poller)
```

## Step 1: CDK Pipe + Rule (`cdk/lib/dynamo-stream-event.ts`)

Add a **Pipe** that filters DynamoDB Stream records and a **Rule** that routes the event to the app-shell Lambda.

```typescript
// ── Pipe: <Description> ──────────────────────────────────────
new pipes.CfnPipe(this, "MyEventPipe", {
  name: `ratel-${stage}-my-event-pipe`,
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
            eventName: ["INSERT"],          // INSERT | MODIFY | REMOVE
            dynamodb: {
              NewImage: {
                sk: { S: [{ prefix: "MY_ENTITY#" }] },
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
      detailType: "MyEventType",
    },
    inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
  },
});

// ── Rule: Route MyEventType events to app-shell Lambda ───────
new events.Rule(this, "MyEventTypeRule", {
  eventBus,
  description: "Route my events to app-shell for processing",
  eventPattern: {
    source: ["ratel.dynamodb.stream"],
    detailType: ["MyEventType"],
  },
  targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
});
```

### Filter patterns

| Event | Use `NewImage` | Use `OldImage` |
|-------|---------------|----------------|
| INSERT | Yes | No |
| MODIFY | Yes (new state) | Optional (old state) |
| REMOVE | No | Yes |

For REMOVE events, use `OldImage` in both filter and `inputTemplate`:
```typescript
inputTemplate: '{"newImage": <$.dynamodb.OldImage>}',
```

## Step 2: Lambda handler (`app/ratel/src/common/types/event_bridge_envelope.rs`)

1. Add variant to `DetailType` enum (before `Unknown`):

```rust
pub enum DetailType {
    // ... existing variants ...
    MyEventType,
    #[serde(other)]
    Unknown,
}
```

2. Add match arm in `EventBridgeEnvelope::proc()`:

```rust
DetailType::MyEventType => {
    let entity: MyEntity = DetailType::parse_detail(&self.detail)?;
    crate::features::my_feature::services::handle_my_event(entity).await
}
```

`DetailType::parse_detail` deserializes `detail.newImage` from DynamoDB attribute format into the target Rust type.

## Step 3: Local-dev stream handler (`app/ratel/src/common/stream_handler.rs`)

Add a matching branch so the local-dev stream poller handles the same event:

```rust
// In the appropriate event_name match arm (INSERT/MODIFY/REMOVE):
} else if sk.starts_with("MY_ENTITY#") {
    let entity = deserialize(image)?;
    if let Err(e) = crate::features::my_feature::services::handle_my_event(entity).await {
        tracing::error!(error = %e, "stream: MyEventType failed");
    }
}
```

## Naming conventions

| CDK Pipe name | `ratel-${stage}-<kebab-case>-pipe` |
|---|---|
| CDK Rule ID | `<PascalCase>Rule` |
| DetailType variant | `PascalCase` (must match CDK `detailType` string exactly) |
| EventBridge source | Always `"ratel.dynamodb.stream"` |

## Checklist

- [ ] CDK Pipe with correct filter on `sk` prefix and `eventName`
- [ ] CDK Rule routing `detailType` to `props.lambdaFunction`
- [ ] `DetailType` variant added (before `Unknown`)
- [ ] `proc()` match arm calling the handler
- [ ] `stream_handler.rs` branch for local-dev parity
- [ ] `batchSize: 1` only if ordering matters (default `10`)
- [ ] CDK compiles: `cd cdk && npx tsc --noEmit`
- [ ] Rust compiles: `cargo check --features "server,lambda"`
