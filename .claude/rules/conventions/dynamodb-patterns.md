---
globs: ["**/models/**/*.rs", "**/models.rs", "packages/by-macros/**/*.rs"]
---

# DynamoDB Patterns

## DynamoEntity Derive

`DYNAMO_TABLE_PREFIX` env var is required at compile time. Table name = `{prefix}-{table}` (default table: `main`).

### Structure Attributes

| Attribute | Default | Description |
|-----------|---------|-------------|
| `table` | `main` | Table name suffix |
| `pk_name` | `pk` | Partition key field |
| `sk_name` | `sk` | Sort key field (omit to remove) |

### Field Attributes

| Attribute | Description |
|-----------|-------------|
| `prefix` | Indexed value prefix (e.g., `"EMAIL"` → `EMAIL#value`) |
| `index` | GSI name (e.g., `"gsi1"`) |
| `pk` | This field is partition key of the index |
| `sk` | This field is sort key of the index |
| `name` | Generated query function name |

### Example

```rust
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct EmailVerification {
    pub pk: String,
    pub sk: String,
    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    #[dynamo(prefix = "EMAIL", name = "find_by_email_and_code", index = "gsi1", pk)]
    pub email: String,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", name = "find_by_code", pk)]
    pub value: String,
}
```

Generates: `EmailVerification::find_by_email_and_code(email, code)` and `EmailVerification::find_by_code(code)`.

## Critical Rules

- **`#[dynamo(prefix)]` must be model-specific abbreviation** to prevent GSI key collisions. See `docs/dynamo-prefix-convention.md`
- **Never call `.to_string()` on `EntityType` for IDs** — convert to sub entity type first (e.g., `SpacePollEntityType`). See `docs/entity-type-id-convention.md`
- **Never use `i32::MAX` / `i64::MAX` as query limits** — creates unbounded reads

## Single-Table Design

- Partition key (`pk`): `Partition` enum (e.g., `USER#<id>`, `SPACE#<id>`)
- Sort key (`sk`): `EntityType` enum
- GSIs gsi1-gsi6+ for email, username, phone, status, visibility queries
