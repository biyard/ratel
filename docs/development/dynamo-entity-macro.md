# DynamoEntity Macro Guide

The `DynamoEntity` derive macro provides a comprehensive solution for interacting with DynamoDB tables. It automatically generates CRUD methods, query builders, transaction support, and update utilities for DynamoDB entities.

## Table of Contents

- [Overview](#overview)
- [Basic Usage](#basic-usage)
- [Configuration](#configuration)
  - [Struct Attributes](#struct-attributes)
  - [Field Attributes](#field-attributes)
- [Generated Methods](#generated-methods)
  - [Core CRUD Methods](#core-crud-methods)
  - [Query Methods](#query-methods)
  - [Transaction Methods](#transaction-methods)
  - [Updater Methods](#updater-methods)
- [Examples](#examples)
  - [Simple Entity](#simple-entity)
  - [Entity with GSI](#entity-with-gsi)
  - [Multiple Indices](#multiple-indices)
- [Transactions](#transactions)
  - [Transaction Overview](#transaction-overview)
  - [Transaction Use Cases](#transaction-use-cases)
  - [Transaction Best Practices](#transaction-best-practices)
  - [Error Handling](#error-handling)
  - [Advanced Transaction Examples](#advanced-transaction-examples)
- [Best Practices](#best-practices)
- [Testing](#testing)
- [Additional Resources](#additional-resources)

## Overview

The `DynamoEntity` macro simplifies DynamoDB operations by:
- Auto-generating CRUD methods (create, get, update, delete)
- Supporting Global Secondary Indexes (GSI)
- Handling key prefixes for better data organization
- Providing query builders for indexed fields
- Managing table name configuration with environment-based prefixes
- Supporting atomic transactions
- Generating fluent updater API for field modifications

## Basic Usage

```rust
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct User {
    pub pk: String,
    pub sk: String,
    pub username: String,
    pub email: String,
    pub created_at: i64,
}
```

## Configuration

### Struct Attributes

Configure the DynamoDB table and behavior at the struct level:

| Attribute | Description | Default | Required |
|-----------|-------------|---------|----------|
| `table` | Table name (excluding prefix) | `main` | No |
| `result_ty` | Result type for operations | `std::result::Result` | No |
| `error_ctor` | Error constructor | `crate::Error2` | No |
| `pk_name` | Partition key field name | `pk` | No |
| `sk_name` | Sort key field name (use `None` to disable) | `sk` | No |

**Environment Variable:**
- `DYNAMO_TABLE_PREFIX`: Required prefix for table names (e.g., `ratel-local`)

**Example:**
```rust
#[derive(DynamoEntity)]
#[dynamo(
    table = "users",
    result_ty = "crate::Result",
    error_ctor = "crate::Error::DynamoDbError",
    pk_name = "user_id",
    sk_name = "sort_key"
)]
pub struct User {
    // fields...
}
```

If `DYNAMO_TABLE_PREFIX=ratel-local`, the full table name will be `ratel-local-users`.

### Field Attributes

Configure how fields map to DynamoDB attributes and indices:

| Attribute | Description | Example |
|-----------|-------------|---------|
| `prefix` | Prefix for the indexed value | `#[dynamo(prefix = "EMAIL")]` |
| `index` | GSI name | `#[dynamo(index = "gsi1")]` |
| `pk` | Mark as partition key for the index | `#[dynamo(index = "gsi1", pk)]` |
| `sk` | Mark as sort key for the index | `#[dynamo(index = "gsi1", sk)]` |
| `name` | Custom query method name | `#[dynamo(name = "find_by_email")]` |

## Generated Methods

### Core CRUD Methods

```rust
// Get table name
fn table_name() -> String

// Get partition key field name
fn pk_field() -> &'static str

// Get sort key field name (if configured)
fn sk_field() -> Option<&'static str>

// Create a new item
async fn create(&self, client: &aws_sdk_dynamodb::Client) -> Result<()>

// Get an item by keys
async fn get(
    client: &aws_sdk_dynamodb::Client,
    pk: String,
    sk: Option<String>
) -> Result<Option<Self>>

// Update an item
async fn update(&self, client: &aws_sdk_dynamodb::Client) -> Result<()>

// Delete an item
async fn delete(
    client: &aws_sdk_dynamodb::Client,
    pk: String,
    sk: Option<String>
) -> Result<()>
```

### Query Methods

For each index configuration, the macro generates query methods with customizable options:

```rust
// Query by index
async fn find_by_{name}(
    client: &aws_sdk_dynamodb::Client,
    pk_value: String,
    options: {Entity}QueryOption
) -> Result<(Vec<Self>, Option<HashMap<String, AttributeValue>>)>
```

### Transaction Methods

The macro generates three transaction-related methods for atomic operations:

```rust
// Create transaction item
pub fn create_transact_write_item(self) -> aws_sdk_dynamodb::types::TransactWriteItem

// Delete transaction item
pub fn delete_transact_write_item(
    pk: impl std::fmt::Display,
    sk: impl std::fmt::Display  // Only if sort key is configured
) -> aws_sdk_dynamodb::types::TransactWriteItem

// Create updater for transactions
pub fn updater(pk: impl std::fmt::Display, sk: impl std::fmt::Display) -> {Entity}Updater
```

### Updater Methods

The updater provides a fluent API for building update operations:

**For All Field Types:**
```rust
pub fn remove_{field}(self) -> Self
```

**For Numeric Fields (i8, i16, i32, i64, u8, u16, u32, u64, f32, f64):**
```rust
pub fn increase_{field}(self, by: i64) -> Self
pub fn decrease_{field}(self, by: i64) -> Self
```

**Convert to Transaction:**
```rust
pub fn transact_write_item(self) -> aws_sdk_dynamodb::types::TransactWriteItem
```

## Examples

### Simple Entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
#[dynamo(table = "sessions")]
pub struct Session {
    pub pk: String,
    pub sk: String,
    pub user_id: i64,
    pub token: String,
    pub expires_at: i64,
}

impl Session {
    pub fn new(user_id: i64, token: String, expires_at: i64) -> Self {
        Self {
            pk: format!("USER#{}", user_id),
            sk: format!("SESSION#{}", token),
            user_id,
            token,
            expires_at,
        }
    }
}

// Usage
let session = Session::new(123, "abc123".to_string(), 1234567890);
session.create(&dynamodb_client).await?;

let retrieved = Session::get(&dynamodb_client, session.pk.clone(), Some(session.sk.clone())).await?;
```

### Entity with GSI

```rust
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct EmailVerification {
    pub pk: String,
    pub sk: String,

    #[dynamo(prefix = "EMAIL", index = "gsi1", pk, name = "find_by_email_and_code")]
    pub email: String,

    #[dynamo(index = "gsi1", sk)]
    pub value: String,

    pub expired_at: i64,
    pub attempt_count: i32,
}

// Usage
let verifications = EmailVerification::find_by_email_and_code(
    &client,
    format!("EMAIL#{}", email),
    EmailVerificationQueryOption::builder()
        .limit(10)
        .sk("CODE123".to_string())
).await?;
```

### Multiple Indices

```rust
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct Post {
    pub pk: String,
    pub sk: String,

    #[dynamo(prefix = "USER", index = "gsi1", pk, name = "find_by_user")]
    pub user_id: String,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "TAG", index = "gsi2", pk, name = "find_by_tag")]
    pub tag: String,

    #[dynamo(prefix = "SCORE", index = "gsi2", sk)]
    pub score: i32,

    pub title: String,
    pub content: String,
}

// Usage - Query by user
let user_posts = Post::find_by_user(
    &client,
    format!("USER#{}", user_id),
    PostQueryOption::builder()
        .limit(20)
        .sk(format!("TS#{}", timestamp))
).await?;

// Usage - Query by tag
let tagged_posts = Post::find_by_tag(
    &client,
    format!("TAG#{}", tag),
    PostQueryOption::builder()
        .limit(10)
        .sk(format!("SCORE#{}", min_score))
).await?;
```

## Transactions

### Transaction Overview

DynamoDB transactions ensure ACID properties across multiple items:
- **Atomic**: All operations succeed or all fail
- **Consistent**: Data integrity is maintained
- **Isolated**: Concurrent transactions don't interfere
- **Durable**: Committed changes persist

DynamoDB supports up to 100 operations in a single transaction. Each operation can be:
- **Put** - Create or replace an item
- **Update** - Modify specific attributes
- **Delete** - Remove an item
- **ConditionCheck** - Verify conditions without modifying data

### Transaction Use Cases

#### Use Case 1: Like a Post

When a user likes a post, atomically:
1. Increment the post's like count
2. Create a PostLike record

```rust
pub async fn like(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: Partition,
    user_pk: Partition,
) -> Result<(), Error> {
    // Create update transaction for post
    let post_tx = Post::updater(&post_pk, EntityType::Post)
        .increase_likes(1)
        .transact_write_item();

    // Create insert transaction for like record
    let like_tx = PostLike::new(post_pk, user_pk)
        .create_transact_write_item();

    // Execute both operations atomically
    cli.transact_write_items()
        .set_transact_items(Some(vec![post_tx, like_tx]))
        .send()
        .await?;

    Ok(())
}
```

#### Use Case 2: Unlike a Post

When a user unlikes a post, atomically:
1. Decrement the post's like count
2. Delete the PostLike record

```rust
pub async fn unlike(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: Partition,
    user_pk: Partition,
) -> Result<(), Error> {
    let post_tx = Post::updater(&post_pk, EntityType::Post)
        .decrease_likes(1)
        .transact_write_item();

    let like_tx = PostLike::delete_transact_write_item(
        &post_pk,
        EntityType::PostLike(user_pk.to_string()).to_string()
    );

    cli.transact_write_items()
        .set_transact_items(Some(vec![post_tx, like_tx]))
        .send()
        .await?;

    Ok(())
}
```

#### Use Case 3: Transfer Credits

Transfer credits between users atomically:

```rust
pub async fn transfer_credits(
    cli: &aws_sdk_dynamodb::Client,
    from_user: Partition,
    to_user: Partition,
    amount: i64,
) -> Result<(), Error> {
    let decrease_tx = UserBalance::updater(&from_user, EntityType::Balance)
        .decrease_credits(amount)
        .transact_write_item();

    let increase_tx = UserBalance::updater(&to_user, EntityType::Balance)
        .increase_credits(amount)
        .transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![decrease_tx, increase_tx]))
        .send()
        .await?;

    Ok(())
}
```

#### Use Case 4: Multi-Entity Update

Update multiple related entities in a single transaction:

```rust
pub async fn publish_post(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: Partition,
    user_pk: Partition,
) -> Result<(), Error> {
    let now = chrono::Utc::now().timestamp_micros();

    let post_tx = Post::updater(&post_pk, EntityType::Post)
        .update_status(PostStatus::Published)
        .update_published_at(now)
        .transact_write_item();

    let user_tx = UserStats::updater(&user_pk, EntityType::Stats)
        .increase_published_posts(1)
        .transact_write_item();

    let activity = Activity::new(user_pk, ActivityType::PostPublished, post_pk.clone());
    let activity_tx = activity.create_transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![post_tx, user_tx, activity_tx]))
        .send()
        .await?;

    Ok(())
}
```

### Transaction Best Practices

#### 1. Keep Transactions Small
- Limit to necessary operations only
- Maximum 100 operations per transaction
- Each operation consumes write capacity

#### 2. Handle Idempotency
Ensure operations can be safely retried:

```rust
// Good - idempotent
let tx = Post::updater(&post_pk, EntityType::Post)
    .increase_views(1)
    .transact_write_item();

// Be careful - may need additional checks
let like_tx = PostLike::new(post_pk, user_pk).create_transact_write_item();
```

#### 3. Use Appropriate Isolation
Transactions provide serializable isolation, which may impact performance for high-contention items.

#### 4. Optimize for Cost
- Transactions consume 2x write capacity
- Batch related changes when possible
- Consider eventual consistency for non-critical updates

### Error Handling

#### Common Transaction Errors

**TransactionCanceledException:**
- Occurs when a condition check fails
- One or more operations couldn't complete
- Contains details about which operation failed

**ValidationException:**
- Invalid transaction request
- Too many operations (>100)
- Invalid item size (>400KB)

**ProvisionedThroughputExceededException:**
- Insufficient write capacity
- Consider using on-demand billing or increasing capacity

#### Error Handling Pattern

```rust
pub async fn atomic_update(
    cli: &aws_sdk_dynamodb::Client,
    items: Vec<aws_sdk_dynamodb::types::TransactWriteItem>,
) -> Result<(), Error> {
    cli.transact_write_items()
        .set_transact_items(Some(items))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Transaction failed: {:?}", e);

            let err_str = e.to_string();
            if err_str.contains("TransactionCanceledException") {
                Error::TransactionCanceled
            } else if err_str.contains("ConditionalCheckFailed") {
                Error::ConditionFailed
            } else {
                Error::DatabaseError(err_str)
            }
        })?;

    Ok(())
}
```

### Advanced Transaction Examples

#### Conditional Updates

Add conditions to prevent race conditions:

```rust
use aws_sdk_dynamodb::types::{TransactWriteItem, Update};

pub async fn conditional_like(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: Partition,
    user_pk: Partition,
) -> Result<(), Error> {
    let mut post_tx = Post::updater(&post_pk, EntityType::Post)
        .increase_likes(1)
        .transact_write_item();

    // Add condition to update only if likes < 1000
    if let Some(update) = post_tx.update.as_mut() {
        update.condition_expression = Some("likes < :max_likes".to_string());
        update.expression_attribute_values.insert(
            ":max_likes".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N("1000".to_string())
        );
    }

    let like_tx = PostLike::new(post_pk, user_pk).create_transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![post_tx, like_tx]))
        .send()
        .await?;

    Ok(())
}
```

#### Optimistic Locking

Implement version-based optimistic locking:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct Document {
    pub pk: String,
    pub sk: String,
    pub version: i64,
    pub content: String,
    pub updated_at: i64,
}

pub async fn update_document_with_version(
    cli: &aws_sdk_dynamodb::Client,
    doc_pk: String,
    new_content: String,
    expected_version: i64,
) -> Result<(), Error> {
    let now = chrono::Utc::now().timestamp_micros();

    let mut update_tx = Document::updater(&doc_pk, "DOCUMENT".to_string())
        .update_content(new_content)
        .increase_version(1)
        .update_updated_at(now)
        .transact_write_item();

    // Add version check condition
    if let Some(update) = update_tx.update.as_mut() {
        update.condition_expression = Some("version = :expected_version".to_string());
        update.expression_attribute_values.insert(
            ":expected_version".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N(expected_version.to_string())
        );
    }

    cli.transact_write_items()
        .set_transact_items(Some(vec![update_tx]))
        .send()
        .await?;

    Ok(())
}
```

#### Complex Multi-Step Transaction

Handle complex business logic with multiple entities:

```rust
pub async fn complete_order(
    cli: &aws_sdk_dynamodb::Client,
    order_pk: Partition,
    user_pk: Partition,
    items: Vec<OrderItem>,
) -> Result<(), Error> {
    let mut transactions = Vec::new();

    // Update order status
    let order_tx = Order::updater(&order_pk, EntityType::Order)
        .update_status(OrderStatus::Completed)
        .update_completed_at(chrono::Utc::now().timestamp_micros())
        .transact_write_item();
    transactions.push(order_tx);

    // Decrease inventory for each item
    for item in items.iter() {
        let inventory_tx = Inventory::updater(&item.product_pk, EntityType::Inventory)
            .decrease_quantity(item.quantity)
            .transact_write_item();
        transactions.push(inventory_tx);
    }

    // Increase user's order count
    let user_tx = UserStats::updater(&user_pk, EntityType::Stats)
        .increase_completed_orders(1)
        .transact_write_item();
    transactions.push(user_tx);

    // Create order completion notification
    let notification = Notification::new(
        user_pk.clone(),
        NotificationType::OrderCompleted,
        order_pk.clone()
    );
    let notification_tx = notification.create_transact_write_item();
    transactions.push(notification_tx);

    // Execute all operations atomically
    cli.transact_write_items()
        .set_transact_items(Some(transactions))
        .send()
        .await?;

    Ok(())
}
```

## Best Practices

### 1. Key Design
- Always use meaningful prefixes for partition keys to avoid collisions
- Design sort keys to enable range queries
- Use composite keys when you need to query by multiple attributes

```rust
// Good
pub pk: String,  // "USER#123"
pub sk: String,  // "POST#456"

// Not recommended
pub pk: String,  // "123"
pub sk: String,  // "456"
```

### 2. Index Design
- Use prefixes to namespace indexed values
- Design GSIs to support your access patterns
- Keep the number of GSIs minimal (maximum 20 per table)

```rust
#[dynamo(prefix = "EMAIL", index = "gsi1", pk)]
pub email: String,  // Stored as "EMAIL#user@example.com"
```

### 3. Query Options
- Always set appropriate limits to control costs
- Use sort key conditions to filter results efficiently
- Handle pagination with `last_evaluated_key`

```rust
let (results, last_key) = Entity::find_by_field(
    &client,
    pk_value,
    EntityQueryOption::builder()
        .limit(100)
        .sk(start_value)
        .last_evaluated_key(continuation_token)
).await?;

// Continue pagination if needed
if let Some(last_key) = last_key {
    // More results available
}
```

### 4. Environment Configuration
- Set `DYNAMO_TABLE_PREFIX` at build time for different environments:
  - Development: `ratel-local`
  - Staging: `ratel-staging`
  - Production: `ratel-prod`

### 5. Error Handling
- Always handle errors from DynamoDB operations
- Consider retry logic for transient failures
- Log errors with sufficient context

```rust
match entity.create(&client).await {
    Ok(_) => println!("Created successfully"),
    Err(e) => {
        eprintln!("Failed to create entity: {:?}", e);
        // Handle error appropriately
    }
}
```

### 6. Transaction Usage
- Use transactions when consistency across multiple items is critical
- Be mindful that transactions consume 2x write capacity
- Handle transaction-specific errors appropriately
- Consider using condition expressions to prevent race conditions

## Testing

### Basic CRUD Testing

Use LocalStack or DynamoDB Local for testing:

```rust
#[tokio::test]
async fn test_entity_operations() {
    let config = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let client = aws_sdk_dynamodb::Client::from_conf(config);

    // Test CRUD operations
    let entity = MyEntity::new(...);
    entity.create(&client).await.unwrap();

    let retrieved = MyEntity::get(&client, entity.pk.clone(), Some(entity.sk.clone()))
        .await
        .unwrap();
    assert!(retrieved.is_some());
}
```

### Transaction Testing

```rust
#[tokio::test]
async fn test_like_post_transaction() {
    let config = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let client = aws_sdk_dynamodb::Client::from_conf(config);

    let post_pk = Partition::Post("test-post".to_string());
    let user_pk = Partition::User(123);

    // Create initial post
    let mut post = Post::new("Test", "Content", PostType::Post, user_pk.clone());
    post.pk = post_pk.clone();
    post.likes = 0;
    post.create(&client).await.unwrap();

    // Like the post
    Post::like(&client, post_pk.clone(), user_pk.clone()).await.unwrap();

    // Verify post likes increased
    let updated_post = Post::get(&client, &post_pk, Some(EntityType::Post))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_post.likes, 1);

    // Verify like record exists
    let like = PostLike::get(
        &client,
        &post_pk,
        Some(EntityType::PostLike(user_pk.to_string()))
    )
    .await
    .unwrap();
    assert!(like.is_some());
}
```

## Additional Resources

- [AWS DynamoDB Best Practices](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/best-practices.html)
- [DynamoDB Global Secondary Indexes](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.html)
- [DynamoDB Transactions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/transactions.html)
- [Rust AWS SDK Documentation](https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/)
