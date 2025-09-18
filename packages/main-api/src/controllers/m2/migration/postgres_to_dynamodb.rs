use bdk::prelude::*;
use dto::{
    Error, Result, User, JsonSchema, aide,
    by_axum::axum::{
        Json,
        extract::{Query, State},
    },
    sqlx::{PgPool, Row},
};
use serde::{Deserialize, Serialize};
use tracing::{info, error, debug, warn};
use crate::{
    utils::aws::dynamo::DynamoClient as DynamoClientWoTableName,
    config,
};

use dto::{DynamoUser as DtoDynamoUser, UserSortKey};

/// Request parameters for the migration API
#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MigrationQuery {
    /// Number of users to migrate in this batch (default: 100, max: 1000)
    #[schemars(description = "Batch size for migration")]
    pub batch_size: Option<u32>,
    
    /// Starting user ID for migration (for pagination)
    #[schemars(description = "Starting user ID for batch migration")]
    pub start_user_id: Option<i64>,
    
    /// Specific user ID to migrate (overrides batch processing)
    #[schemars(description = "Specific user ID to migrate")]
    pub user_id: Option<i64>,
    
    /// Dry run mode - validate migration without writing to DynamoDB
    #[schemars(description = "Dry run mode - validate only")]
    pub dry_run: Option<bool>,
}

/// Response from the migration API
#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MigrationResponse {
    /// Number of users successfully migrated
    pub migrated_count: u32,
    
    /// Number of users that failed migration
    pub failed_count: u32,
    
    /// List of user IDs that failed migration
    pub failed_user_ids: Vec<i64>,
    
    /// Error messages for failed migrations
    pub errors: Vec<String>,
    
    /// Next starting user ID for pagination (if applicable)
    pub next_start_user_id: Option<i64>,
    
    /// Whether this was a dry run
    pub dry_run: bool,
    
    /// Total processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Migration statistics response
#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MigrationStatsResponse {
    /// Total number of users in PostgreSQL
    pub total_postgres_users: i64,
    
    /// Total number of users in DynamoDB
    pub total_dynamo_users: i64,
    
    /// Number of users pending migration
    pub pending_migration: i64,
    
    /// Last migrated user ID
    pub last_migrated_user_id: Option<i64>,
}

/// Main migration handler
pub async fn migrate_users_handler(
    State(pool): State<PgPool>,
    Query(query): Query<MigrationQuery>,
) -> Result<Json<MigrationResponse>> {
    let start_time = std::time::Instant::now();
    
    info!("Starting user migration with params: {:?}", query);
    
    let batch_size = query.batch_size.unwrap_or(100).min(1000);
    let dry_run = query.dry_run.unwrap_or(false);
    
    let mut migrated_count = 0u32;
    let mut failed_count = 0u32;
    let mut failed_user_ids = Vec::new();
    let mut errors = Vec::new();
    let mut next_start_user_id = None;
    
    // Initialize DynamoDB client
    let conf = config::get();
    let dynamo_client = DynamoClient::new(&conf.dual_write.table_name);
    
    if let Some(user_id) = query.user_id {
        // Migrate single user
        match migrate_single_user(&pool, &dynamo_client, user_id, dry_run).await {
            Ok(()) => {
                migrated_count = 1;
                info!("Successfully migrated user ID: {}", user_id);
            }
            Err(e) => {
                failed_count = 1;
                failed_user_ids.push(user_id);
                errors.push(format!("User {}: {}", user_id, e));
                error!("Failed to migrate user {}: {}", user_id, e);
            }
        }
    } else {
        // Batch migration
        let start_id = query.start_user_id.unwrap_or(0);
        
        match migrate_user_batch(&pool, &dynamo_client, start_id, batch_size, dry_run).await {
            Ok(result) => {
                migrated_count = result.migrated_count;
                failed_count = result.failed_count;
                failed_user_ids = result.failed_user_ids;
                errors = result.errors;
                next_start_user_id = result.next_start_user_id;
            }
            Err(e) => {
                error!("Batch migration failed: {}", e);
                errors.push(format!("Batch migration error: {}", e));
            }
        }
    }
    
    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    
    let response = MigrationResponse {
        migrated_count,
        failed_count,
        failed_user_ids,
        errors,
        next_start_user_id,
        dry_run,
        processing_time_ms,
    };
    
    info!("Migration completed: {:?}", response);
    
    Ok(Json(response))
}
struct DynamoClient {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

impl DynamoClient {
    pub fn new(table_name: &str) -> Self {
        let cli = DynamoClientWoTableName::new();
        Self { client: cli.client, table_name: table_name.to_string() }
    }
}
/// Get migration statistics
pub async fn migration_stats_handler(
    State(pool): State<PgPool>,
) -> Result<Json<MigrationStatsResponse>> {
    info!("Fetching migration statistics");
    
    // Count total users in PostgreSQL
    let total_postgres_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            error!("Failed to count PostgreSQL users: {}", e);
            Error::DatabaseException(e.to_string())
        })?;
    
    // Initialize DynamoDB client and count users
    let conf = config::get();
    let dynamo_client = DynamoClient::new(&conf.dual_write.table_name);
    
    let total_dynamo_users = count_dynamo_users(&dynamo_client).await?;
    
    // Get last migrated user ID (highest user_id in DynamoDB)
    let last_migrated_user_id = get_last_migrated_user_id(&dynamo_client).await?;
    
    let pending_migration = total_postgres_users - total_dynamo_users;
    
    let stats = MigrationStatsResponse {
        total_postgres_users,
        total_dynamo_users,
        pending_migration: pending_migration.max(0),
        last_migrated_user_id,
    };
    
    info!("Migration stats: {:?}", stats);
    
    Ok(Json(stats))
}

/// Batch migration result
#[derive(Debug)]
struct BatchMigrationResult {
    migrated_count: u32,
    failed_count: u32,
    failed_user_ids: Vec<i64>,
    errors: Vec<String>,
    next_start_user_id: Option<i64>,
}

/// Migrate a batch of users
async fn migrate_user_batch(
    pool: &PgPool,
    dynamo_client: &DynamoClient,
    start_user_id: i64,
    batch_size: u32,
    dry_run: bool,
) -> Result<BatchMigrationResult> {
    info!("Migrating user batch starting from ID: {}, size: {}", start_user_id, batch_size);
    
    // Fetch users from PostgreSQL using raw SQL
    let users = sqlx::query(
        r#"
        SELECT 
            id, created_at, updated_at, nickname, principal, email, profile_url,
            term_agreed, informed_agreed, user_type, parent_id,
            username, evm_address, password, membership,
            theme, referral_code, phone_number, telegram_id
        FROM users 
        WHERE id >= $1 
        ORDER BY id ASC 
        LIMIT $2
        "#
    )
    .bind(start_user_id)
    .bind(batch_size as i64)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch users from PostgreSQL: {}", e);
        Error::DatabaseException(e.to_string())
    })?;
    
    // Convert rows to User structs
    let mut user_list = Vec::new();
    for row in users {
        let user = User {
            id: row.get("id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            nickname: row.get("nickname"),
            principal: row.get("principal"),
            email: row.get("email"),
            profile_url: row.get("profile_url"),
            term_agreed: row.get("term_agreed"),
            informed_agreed: row.get("informed_agreed"),
            user_type: dto::UserType::Individual, // Default for now
            parent_id: row.get("parent_id"),
            username: row.get("username"),
            evm_address: row.get::<Option<String>, _>("evm_address").unwrap_or_default(),
            password: row.get::<Option<String>, _>("password").unwrap_or_default(),
            membership: dto::Membership::Free, // Default for now
            theme: None,
            referral_code: row.get::<Option<String>, _>("referral_code").unwrap_or_default(),
            phone_number: row.get("phone_number"),
            telegram_id: row.get("telegram_id"),
            // Set default values for fields not fetched
            followers_count: 0,
            followings_count: 0,
            groups: Vec::new(),
            teams: Vec::new(),
            html_contents: String::new(),
            followers: Vec::new(),
            followings: Vec::new(),
            badges: Vec::new(),
            points: 0,
            phone: String::new(),
            telegram_raw: String::new(),
            industry: Vec::new(),
        };
        user_list.push(user);
    }
    let users = user_list;
    
    if users.is_empty() {
        info!("No more users to migrate");
        return Ok(BatchMigrationResult {
            migrated_count: 0,
            failed_count: 0,
            failed_user_ids: Vec::new(),
            errors: Vec::new(),
            next_start_user_id: None,
        });
    }
    
    let mut migrated_count = 0u32;
    let mut failed_count = 0u32;
    let mut failed_user_ids = Vec::new();
    let mut errors = Vec::new();
    
    let last_user_id = users.last().map(|u| u.id);
    
    for user in users {
        match migrate_user_to_dynamo(dynamo_client, &user, dry_run).await {
            Ok(()) => {
                migrated_count += 1;
                debug!("Migrated user ID: {}", user.id);
            }
            Err(e) => {
                failed_count += 1;
                failed_user_ids.push(user.id);
                errors.push(format!("User {}: {}", user.id, e));
                warn!("Failed to migrate user {}: {}", user.id, e);
            }
        }
    }
    
    let next_start_user_id = last_user_id.map(|id| id + 1);
    
    Ok(BatchMigrationResult {
        migrated_count,
        failed_count,
        failed_user_ids,
        errors,
        next_start_user_id,
    })
}

/// Migrate a single user
async fn migrate_single_user(
    pool: &PgPool,
    dynamo_client: &DynamoClient,
    user_id: i64,
    dry_run: bool,
) -> Result<()> {
    info!("Migrating single user ID: {}", user_id);
    
    // Fetch user from PostgreSQL using regular query
    let row = sqlx::query(
        r#"
        SELECT 
            id, created_at, updated_at, nickname, principal, email, profile_url,
            term_agreed, informed_agreed, user_type, parent_id,
            username, evm_address, password, membership,
            theme, referral_code, phone_number, telegram_id
        FROM users 
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch user {} from PostgreSQL: {}", user_id, e);
        Error::NotFound
    })?;
    
    // Convert row to User struct
    let user = User {
        id: row.get("id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        nickname: row.get("nickname"),
        principal: row.get("principal"),
        email: row.get("email"),
        profile_url: row.get("profile_url"),
        term_agreed: row.get("term_agreed"),
        informed_agreed: row.get("informed_agreed"),
        user_type: dto::UserType::Individual, // Default for now
        parent_id: row.get("parent_id"),
        username: row.get("username"),
        evm_address: row.get::<Option<String>, _>("evm_address").unwrap_or_default(),
        password: row.get::<Option<String>, _>("password").unwrap_or_default(),
        membership: dto::Membership::Free, // Default for now
        theme: None,
        referral_code: row.get::<Option<String>, _>("referral_code").unwrap_or_default(),
        phone_number: row.get("phone_number"),
        telegram_id: row.get("telegram_id"),
        // Set default values for fields not fetched
        followers_count: 0,
        followings_count: 0,
        groups: Vec::new(),
        teams: Vec::new(),
        html_contents: String::new(),
        followers: Vec::new(),
        followings: Vec::new(),
        badges: Vec::new(),
        points: 0,
        phone: String::new(),
        telegram_raw: String::new(),
        industry: Vec::new(),
    };
    
    migrate_user_to_dynamo(dynamo_client, &user, dry_run).await
}

/// Convert PostgreSQL User to DynamoDB format and store it
async fn migrate_user_to_dynamo(
    dynamo_client: &DynamoClient,
    user: &User,
    dry_run: bool,
) -> Result<()> {
    // Convert PostgreSQL User to DynamoDB format
    let dynamo_user = convert_user_to_dynamo(user)?;
    
    if dry_run {
        debug!("DRY RUN: Would migrate user {} to DynamoDB", user.id);
        return Ok(());
    }
    
    // Check if user already exists in DynamoDB
    if user_exists_in_dynamo(dynamo_client, user.id).await? {
        debug!("User {} already exists in DynamoDB, skipping", user.id);
        return Ok(());
    }
    
    // Store in DynamoDB
    let item = dynamo_user.to_dynamo_item();
    
    dynamo_client.client
        .put_item()
        .table_name(&dynamo_client.table_name)
        .set_item(Some(item))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to store user {} in DynamoDB: {}", user.id, e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;
    
    debug!("Successfully stored user {} in DynamoDB", user.id);
    Ok(())
}

/// Convert PostgreSQL User to DynamoDB User
fn convert_user_to_dynamo(user: &User) -> Result<DtoDynamoUser> {
    // Map PostgreSQL User fields to DynamoDB User
    let dynamo_user = DtoDynamoUser {
        pk: format!("USER#{}", user.id),
        sk: UserSortKey::User,
        user_id: user.id,
        telegram_id: user.telegram_id.map(|id| id.to_string()),
        evm_address: if user.evm_address.is_empty() { 
            None 
        } else { 
            Some(user.evm_address.clone()) 
        },
        username: user.username.clone(),
        created_at: user.created_at,
        gsi1_pk: Some(format!("USERNAME#{}", user.username)),
        gsi1_sk: Some("USER".to_string()),
    };
    
    Ok(dynamo_user)
}

/// Check if user exists in DynamoDB
async fn user_exists_in_dynamo(dynamo_client: &DynamoClient, user_id: i64) -> Result<bool> {
    let pk = format!("USER#{}", user_id);
    
    let result = dynamo_client.client
        .get_item()
        .table_name(&dynamo_client.table_name)
        .key("PK", aws_sdk_dynamodb::types::AttributeValue::S(pk))
        .key("SK", aws_sdk_dynamodb::types::AttributeValue::S("USER".to_string()))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to check if user {} exists in DynamoDB: {}", user_id, e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;
    
    Ok(result.item.is_some())
}

/// Count total users in DynamoDB
async fn count_dynamo_users(dynamo_client: &DynamoClient) -> Result<i64> {
    // Use scan to count all users (not efficient for large datasets, but works for migration tracking)
    let mut total_count = 0i64;
    let mut last_evaluated_key = None;
    
    loop {
        let mut request = dynamo_client.client
            .scan()
            .table_name(&dynamo_client.table_name)
            .filter_expression("begins_with(PK, :pk_prefix)")
            .expression_attribute_values(":pk_prefix", aws_sdk_dynamodb::types::AttributeValue::S("USER#".to_string()))
            .select(aws_sdk_dynamodb::types::Select::Count);
        
        if let Some(key) = last_evaluated_key {
            request = request.set_exclusive_start_key(Some(key));
        }
        
        let response = request.send().await
            .map_err(|e| {
                error!("Failed to scan DynamoDB for user count: {}", e);
                Error::Unknown(format!("DynamoDB error: {}", e))
            })?;
        
        total_count += response.count() as i64;
        
        if response.last_evaluated_key().is_none() {
            break;
        }
        
        last_evaluated_key = response.last_evaluated_key().cloned();
    }
    
    Ok(total_count)
}

/// Get the highest user_id that has been migrated to DynamoDB
async fn get_last_migrated_user_id(dynamo_client: &DynamoClient) -> Result<Option<i64>> {
    // This is a simplified approach - in practice you might want to maintain a separate migration state table
    let result = dynamo_client.client
        .scan()
        .table_name(&dynamo_client.table_name)
        .filter_expression("begins_with(PK, :pk_prefix)")
        .expression_attribute_values(":pk_prefix", aws_sdk_dynamodb::types::AttributeValue::S("USER#".to_string()))
        .projection_expression("user_id")
        .send()
        .await
        .map_err(|e| {
            error!("Failed to scan DynamoDB for last user ID: {}", e);
            Error::Unknown(format!("DynamoDB error: {}", e))
        })?;
    
    let mut max_user_id = None;
    
    for item in result.items() {
        if let Some(aws_sdk_dynamodb::types::AttributeValue::N(user_id_str)) = item.get("user_id") {
            if let Ok(user_id) = user_id_str.parse::<i64>() {
                max_user_id = Some(max_user_id.unwrap_or(0).max(user_id));
            }
        }
    }
    
    Ok(max_user_id)
}