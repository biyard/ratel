# EntityType to ID String Convention

## Problem

`EntityType` is an enum whose `Display` implementation outputs the full DynamoDB sort key with prefix:

```
EntityType::SpacePoll("uuid123").to_string()  →  "SPACE_POLL#uuid123"
```

This is **not** the raw ID you want when passing action IDs to URLs, components, or storing as plain identifiers. You need just `"uuid123"`.

## Solution: Convert via Sub EntityType

The `SubPartition` derive macro generates a newtype struct for each `EntityType` variant (e.g., `SpacePollEntityType`). These newtypes implement `Display` to output **only the inner value** without the prefix.

### Pattern

```rust
// Step 1: Convert EntityType to its sub entity type
let id: SpacePollEntityType = entity.sk.clone().into();

// Step 2: Get the raw ID string
id.to_string()  // → "uuid123"

// Other patter with from
SpacePollEntityType::from(entity.sk);
```

### Comparison

| Expression | Output |
|---|---|
| `EntityType::SpacePoll("uuid123").to_string()` | `"SPACE_POLL#uuid123"` |
| `SpacePollEntityType("uuid123".into()).to_string()` | `"uuid123"` |

### Reverse: String → Sub EntityType → EntityType

```rust
// Parse raw ID string into sub entity type
let poll_id: SpacePollEntityType = "uuid123".parse().unwrap();

// Convert to EntityType for DynamoDB queries
let entity_type: EntityType = poll_id.into();
// → EntityType::SpacePoll("uuid123")
```

## Sub EntityType Reference

| EntityType Variant | Sub EntityType | Has Inner Value |
|---|---|---|
| `SpacePoll(String)` | `SpacePollEntityType` | Yes |
| `SpaceQuiz(String)` | `SpaceQuizEntityType` | Yes |
| `SpacePost(String)` | `SpacePostEntityType` | Yes |
| `SpaceActionFollow(String)` | `SpaceActionFollowEntityType` | Yes |
| `SpaceSubscription` | `SpaceSubscriptionEntityType` | No (unit) |

## When to Use

- **Storing action IDs** in `SpaceAction.pk` (which uses `CompositePartition<Partition, String>`)
- **Passing IDs to URL path params** (e.g., `/api/spaces/{space_id}/actions/{action_id}`)
- **Passing IDs to component props** (e.g., `action_id: ReadSignal<String>`)
- **Displaying IDs in UI** where the prefix is not desired

## When NOT to Use

- **DynamoDB queries** — use the full `EntityType` (the DynamoDB sort key includes the prefix)
- **Constructing DynamoDB keys** — use `EntityType` directly
