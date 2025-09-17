# DynamoEntity Derive Macro

The `DynamoEntity` derive macro automatically generates CRUD operations and query builders for Amazon DynamoDB entities in Rust. It provides a high-level, type-safe interface for interacting with DynamoDB tables.

## Table of Contents

- [Quick Start](#quick-start)
- [Struct Attributes](#struct-attributes)
- [Field Attributes](#field-attributes)
- [Generated Methods](#generated-methods)
- [Query Operations](#query-operations)
- [Update Operations](#update-operations)
- [Global Secondary Indexes (GSI)](#global-secondary-indexes-gsi)
- [Error Handling](#error-handling)
- [Examples](#examples)

## Quick Start

```rust
use by_macros::DynamoEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
#[dynamo(table = "users")]
pub struct User {
    pub pk: String,
    pub sk: String,
    pub username: String,
    pub email: String,
    pub created_at: i64,
}
```

## Struct Attributes

Configure the DynamoEntity behavior using struct-level attributes:

### `#[dynamo(...)]`

| Attribute    | Description                                | Default                |
|:-------------|:-------------------------------------------|:-----------------------|
| `table`      | Table name suffix (without prefix)        | `"main"`              |
| `result_ty`  | Custom Result type                         | `std::result::Result` |
| `error_ctor` | Custom Error constructor                   | `crate::Error2`       |
| `pk_name`    | Partition key field name                   | `"pk"`                |
| `sk_name`    | Sort key field name (use `"none"` to disable) | `"sk"`            |

```rust
#[derive(DynamoEntity)]
#[dynamo(
    table = "users",
    result_ty = "crate::Result",
    error_ctor = "crate::Error::DynamoDbError",
    pk_name = "id",
    sk_name = "none"  // For PK-only tables
)]
pub struct User {
    pub id: String,
    pub username: String,
}
```

## Field Attributes

Configure individual fields for Global Secondary Index (GSI) support:

### `#[dynamo(...)]`

| Attribute | Description                          | Required With |
|:----------|:-------------------------------------|:--------------|
| `index`   | GSI name (e.g., "gsi1", "gsi2")    | `pk` or `sk`  |
| `pk`      | Mark field as GSI partition key      | `index`       |
| `sk`      | Mark field as GSI sort key           | `index`       |
| `prefix`  | Prefix for the indexed value         | -             |
| `name`    | Custom query function name           | -             |

```rust
#[derive(DynamoEntity)]
pub struct Post {
    pub pk: String,
    pub sk: String,

    #[dynamo(prefix = "USER", index = "gsi1", pk, name = "find_by_user")]
    pub user_id: String,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "STATUS", index = "gsi2", pk)]
    pub status: String,
}
```

## Generated Methods

### Core CRUD Operations

The macro generates the following methods for your struct:

#### Static Methods

```rust
impl MyEntity {
    // Table metadata
    pub fn table_name() -> &'static str;
    pub fn pk_field() -> &'static str;
    pub fn sk_field() -> Option<&'static str>;

    // CRUD operations
    pub async fn get(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display,
        sk: Option<impl std::fmt::Display>,  // Only if SK is configured
    ) -> Result<Option<Self>, Error>;

    pub async fn delete(
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display,
        sk: Option<impl std::fmt::Display>,  // Only if SK is configured
    ) -> Result<(), Error>;
}
```

#### Instance Methods

```rust
impl MyEntity {
    // Create/Update
    pub async fn create(&self, cli: &aws_sdk_dynamodb::Client) -> Result<(), Error>;

    // Index field computation (for GSI attributes)
    pub fn indexed_fields(
        &self,
        item: HashMap<String, AttributeValue>
    ) -> HashMap<String, AttributeValue>;
}
```

### Key Composers

For each GSI field with a prefix, composer functions are generated:

```rust
impl MyEntity {
    pub fn compose_gsi1_pk(key: impl std::fmt::Display) -> String;
    pub fn compose_gsi1_sk(key: impl std::fmt::Display) -> String;
}
```

## Query Operations

### Query Option Builder

Each entity gets a `{EntityName}QueryOption` struct for configuring queries:

```rust
pub struct MyEntityQueryOption {
    pub sk: Option<String>,         // Only if SK is configured
    pub bookmark: Option<String>,   // For pagination
    pub limit: i32,                 // Query limit (default: 10)
    pub scan_index_forward: bool,   // Sort order (default: false)
}

impl MyEntityQueryOption {
    pub fn builder() -> Self;
    pub fn sk(mut self, sk: String) -> Self;
    pub fn bookmark(mut self, bookmark: String) -> Self;
    pub fn limit(mut self, limit: i32) -> Self;
    pub fn scan_index_forward(mut self, scan_index_forward: bool) -> Self;
}
```

### GSI Query Functions

For each GSI configuration with a `name` attribute, a query function is generated:

```rust
impl MyEntity {
    pub async fn find_by_user(  // Custom name from field attribute
        cli: &aws_sdk_dynamodb::Client,
        pk: impl std::fmt::Display,
        opt: MyEntityQueryOption,
    ) -> Result<(Vec<Self>, Option<String>), Error>;
}
```

### Pagination Utilities

```rust
impl MyEntity {
    // Encode last evaluated key for pagination
    pub fn encode_lek_all(
        lek: &HashMap<String, AttributeValue>
    ) -> String;

    // Decode bookmark for pagination
    pub fn decode_bookmark_all(
        bookmark: &str
    ) -> Result<HashMap<String, AttributeValue>, Error>;
}
```

## Update Operations

### Update Builder

Each entity gets a `{EntityName}Updater` struct for fluent updates:

```rust
impl MyEntity {
    pub fn updater(
        pk: impl std::fmt::Display,
        sk: impl std::fmt::Display,  // Only if SK is configured
    ) -> MyEntityUpdater;
}

impl MyEntityUpdater {
    // For each non-key field
    pub fn with_{field_name}(self, value: FieldType) -> Self;
    pub fn remove_{field_name}(self) -> Self;

    // For numeric fields only
    pub fn increase_{field_name}(self, by: i64) -> Self;
    pub fn decrease_{field_name}(self, by: i64) -> Self;

    // Execute the update
    pub async fn execute(self, cli: &aws_sdk_dynamodb::Client) -> Result<(), Error>;
}
```

## Global Secondary Indexes (GSI)

### Index Configuration

GSIs are configured using field attributes. Each field can participate in multiple indexes:

```rust
#[derive(DynamoEntity)]
pub struct EmailVerification {
    pub pk: String,
    pub sk: String,

    #[dynamo(prefix = "EMAIL", index = "gsi1", pk, name = "find_by_email_and_code")]
    pub email: String,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", pk, name = "find_by_code")]
    pub value: String,

    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
}
```

### Index Naming Convention

- GSI names in DynamoDB: `{base_name}-index` (e.g., "gsi1-index")
- GSI attribute names: `{base_name}_{pk|sk}` (e.g., "gsi1_pk", "gsi1_sk")

### Automatic Index Field Generation

The macro automatically populates GSI fields in the DynamoDB item:

- **With prefix**: `gsi1_pk = "EMAIL#{email}"`
- **Without prefix**: `gsi1_sk = "{value}"`

## Error Handling

### Default Error Configuration

```rust
// Default error handling
type Result<T> = std::result::Result<T, crate::Error2>;
```

### Custom Error Configuration

```rust
#[derive(DynamoEntity)]
#[dynamo(
    result_ty = "anyhow::Result",
    error_ctor = "anyhow::Error"
)]
pub struct MyEntity {
    // ...
}
```

## Examples

### Basic Entity (PK + SK)

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct User {
    pub pk: String,      // Partition key
    pub sk: String,      // Sort key
    pub username: String,
    pub email: String,
    pub created_at: i64,
}

// Usage
let user = User { /* ... */ };
user.create(&client).await?;

let found_user = User::get(&client, "USER#123", Some("PROFILE")).await?;
User::delete(&client, "USER#123", Some("PROFILE")).await?;
```

### PK-Only Entity

```rust
#[derive(DynamoEntity)]
#[dynamo(sk_name = "none")]
pub struct Counter {
    pub pk: String,
    pub count: i64,
}

// Usage
let counter = Counter::get(&client, "COUNTER#daily").await?;
```

### Entity with GSI

```rust
#[derive(DynamoEntity)]
pub struct Post {
    pub pk: String,
    pub sk: String,

    #[dynamo(prefix = "USER", index = "gsi1", pk, name = "find_by_author")]
    pub author_id: String,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "STATUS", index = "gsi2", pk, name = "find_by_status")]
    pub status: String,

    pub title: String,
    pub content: String,
}

// Usage - Query by author
let posts = Post::find_by_author(
    &client,
    Post::compose_gsi1_pk("user123"),
    PostQueryOption::builder()
        .limit(20)
        .scan_index_forward(true)
).await?;

// Usage - Query by status with pagination
let (posts, bookmark) = Post::find_by_status(
    &client,
    Post::compose_gsi2_pk("published"),
    PostQueryOption::builder()
        .limit(10)
        .bookmark("previous_bookmark".to_string())
).await?;
```

### Update Operations

```rust
// Fluent update API
User::updater("USER#123", "PROFILE")
    .with_username("new_username".to_string())
    .with_email("new@email.com".to_string())
    .increase_login_count(1)
    .execute(&client)
    .await?;

// Remove fields
User::updater("USER#123", "PROFILE")
    .remove_optional_field()
    .execute(&client)
    .await?;
```

### Environment Configuration

The table name is constructed from the environment variable `DYNAMO_TABLE_PREFIX`:

```bash
export DYNAMO_TABLE_PREFIX="ratel-local"
```

```rust
#[derive(DynamoEntity)]
#[dynamo(table = "users")]
pub struct User { /* ... */ }

// Generates table name: "ratel-local-users"
assert_eq!(User::table_name(), "ratel-local-users");
```

## Best Practices

1. **Use meaningful GSI names**: Choose descriptive names like "gsi1", "gsi2" that map to your access patterns
2. **Apply prefixes consistently**: Use prefixes to avoid key collisions (e.g., "USER#", "POST#")
3. **Leverage composite sort keys**: Combine multiple values in sort keys for range queries
4. **Handle pagination**: Always check for bookmarks in query results for large datasets
5. **Use builders**: Leverage the generated query and update builders for cleaner code
6. **Type safety**: The macro preserves Rust's type safety while providing DynamoDB integration

## Table Schema Requirements

Ensure your DynamoDB table has the following configuration:

### Primary Table
- Partition Key: Configured via `pk_name` (default: "pk")
- Sort Key: Configured via `sk_name` (default: "sk", optional)

### Global Secondary Indexes
For each GSI used in field attributes:
- GSI Name: `{index_name}-index` (e.g., "gsi1-index")
- Partition Key: `{index_name}_pk` (e.g., "gsi1_pk")
- Sort Key: `{index_name}_sk` (e.g., "gsi1_sk")

### Example CloudFormation/CDK Configuration

```yaml
# For an entity with gsi1 and gsi2 indexes
GlobalSecondaryIndexes:
  - IndexName: gsi1-index
    KeySchema:
      - AttributeName: gsi1_pk
        KeyType: HASH
      - AttributeName: gsi1_sk
        KeyType: RANGE
    ProjectionType: ALL

  - IndexName: gsi2-index
    KeySchema:
      - AttributeName: gsi2_pk
        KeyType: HASH
      - AttributeName: gsi2_sk
        KeyType: RANGE
    ProjectionType: ALL
```