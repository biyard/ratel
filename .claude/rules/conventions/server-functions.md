---
globs: ["app/ratel/src/**/controllers/*.rs"]
---

# Server Functions

Conventions for implementing server functions (controllers) in `app/ratel/src/features/`.

## Naming: pk, sk, id

| Term | Type | Has prefix | Example |
|------|------|-----------|---------|
| `pk` | `Partition` | Yes | `SPACE#abc123` |
| `sk` | `EntityType` | Yes | `SPACE_POLL#xyz` |
| `id` | SubPartition (`{Name}Partition` or `{Name}EntityType`) | No | `abc123` |

Path parameters, request bodies, and response DTOs must use **`id`** (SubPartition types), not `pk` or `sk`.

```rust
// GOOD — uses "id" naming with SubPartition types
fn update_poll(space_id: SpacePartition, poll_id: SpacePollEntityType)

// BAD — uses "pk"/"sk" naming (implies raw Partition/EntityType with prefix)
fn update_poll(space_pk: SpacePartition, poll_sk: SpacePollEntityType)
```

## SubPartition Types

Always use SubPartition types — not raw `Partition` or `EntityType` — for path parameters, request bodies, and response DTOs.

### How SubPartition Works

`#[derive(SubPartition)]` on `Partition` and `EntityType` auto-generates typed wrapper structs:

| Source Enum | Variant | Generated SubPartition | Serializes as |
|-------------|---------|----------------------|---------------|
| `Partition` | `Space(String)` | `SpacePartition(String)` | just the ID (no `SPACE#` prefix) |
| `Partition` | `Feed(String)` | `FeedPartition(String)` | just the ID (no `FEED#` prefix) |
| `Partition` | `Team(String)` | `TeamPartition(String)` | just the ID (no `TEAM#` prefix) |
| `EntityType` | `SpacePoll(String)` | `SpacePollEntityType(String)` | just the ID (no `SPACE_POLL#` prefix) |
| `EntityType` | `SpaceQuiz(String)` | `SpaceQuizEntityType(String)` | just the ID (no `SPACE_QUIZ#` prefix) |

SubPartition types auto-strip/add the prefix during deserialization/serialization, so clients pass just the ID — no URL-encoding of `#` needed.

### Conversion

```rust
// SubPartition → Partition (for DynamoDB queries)
let space_pk: Partition = space_partition.into();

// Partition → SubPartition (for response DTOs)
let space_partition: SpacePartition = partition.into();
```

### Controller Pattern

```rust
#[post("/api/spaces/{space_id}/polls/{poll_id}", role: SpaceUserRole)]
pub async fn update_poll(
    space_id: SpacePartition,        // path param — client sends just the ID
    poll_id: SpacePollEntityType,    // path param — client sends just the ID
    req: UpdatePollRequest,
) -> Result<String> {
    let space_pk: Partition = space_id.into();  // convert to pk for DynamoDB
    // ...
}
```

### Response DTO Pattern

```rust
pub struct SpaceResponse {
    pub id: SpacePartition,       // serializes as just the ID
    pub post_id: FeedPartition,   // serializes as just the ID
    pub title: String,
}
```

### Rules

- **Path params**: always SubPartition (`SpacePartition`, `FeedPartition`, `{Name}EntityType`)
- **Request/response DTOs**: always SubPartition for partition/entity fields
- **Internal logic**: convert to `Partition`/`EntityType` via `.into()` for DynamoDB operations
- **Never** expose raw `Partition` or `EntityType` in API interfaces
