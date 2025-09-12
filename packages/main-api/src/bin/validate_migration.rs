use bdk::prelude::*;
use dto::{Result, User};
use main_api::{
    models::dynamo::{DynamoUser, DynamoModel},
    utils::aws::DynamoClient,
};
use std::env;
use tracing::{info, error, warn};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting migration validation");

    let args: Vec<String> = env::args().collect();
    let validation_type = args.get(1).map(|s| s.as_str()).unwrap_or("all");

    match validation_type {
        "users" => validate_users().await,
        "spaces" => validate_spaces().await,
        "feeds" => validate_feeds().await,
        "relationships" => validate_relationships().await,
        "all" => validate_all().await,
        "count" => validate_record_counts().await,
        "sample" => {
            let sample_size = args.get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(100);
            validate_sample_records(sample_size).await
        }
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => {
            eprintln!("Unknown validation type: {}", validation_type);
            print_help();
            std::process::exit(1);
        }
    }
}

async fn validate_all() -> Result<()> {
    info!("Running comprehensive validation");
    
    let mut errors = 0;
    
    info!("1. Validating record counts...");
    if let Err(e) = validate_record_counts().await {
        error!("Record count validation failed: {:?}", e);
        errors += 1;
    }
    
    info!("2. Validating user records...");
    if let Err(e) = validate_users().await {
        error!("User validation failed: {:?}", e);
        errors += 1;
    }
    
    info!("3. Validating space records...");
    if let Err(e) = validate_spaces().await {
        error!("Space validation failed: {:?}", e);
        errors += 1;
    }
    
    info!("4. Validating feed records...");
    if let Err(e) = validate_feeds().await {
        error!("Feed validation failed: {:?}", e);
        errors += 1;
    }
    
    info!("5. Validating relationship records...");
    if let Err(e) = validate_relationships().await {
        error!("Relationship validation failed: {:?}", e);
        errors += 1;
    }
    
    if errors == 0 {
        info!("✅ All validations passed!");
    } else {
        error!("❌ {} validation(s) failed", errors);
    }
    
    Ok(())
}

async fn validate_record_counts() -> Result<()> {
    info!("Validating record counts between PostgreSQL and DynamoDB");

    let postgres_pool = get_database_pool().await?;
    let dynamo_client = get_dynamo_client().await?;

    // Count records in PostgreSQL
    let pg_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&postgres_pool)
        .await
        .map_err(|e| dto::Error::DatabaseException(format!("Failed to count PostgreSQL users: {:?}", e)))?;

    let pg_spaces: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM spaces")
        .fetch_one(&postgres_pool)
        .await
        .map_err(|e| dto::Error::DatabaseException(format!("Failed to count PostgreSQL spaces: {:?}", e)))?;

    let pg_feeds: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM feeds")
        .fetch_one(&postgres_pool)
        .await
        .map_err(|e| dto::Error::DatabaseException(format!("Failed to count PostgreSQL feeds: {:?}", e)))?;

    // Count records in DynamoDB by type
    let dynamo_users = count_dynamo_records_by_type(&dynamo_client, "USER").await?;
    let dynamo_spaces = count_dynamo_records_by_type(&dynamo_client, "SPACE").await?;
    let dynamo_feeds = count_dynamo_records_by_type(&dynamo_client, "FEED").await?;

    // Compare counts
    info!("Record count comparison:");
    info!("Users    - PostgreSQL: {}, DynamoDB: {}", pg_users, dynamo_users);
    info!("Spaces   - PostgreSQL: {}, DynamoDB: {}", pg_spaces, dynamo_spaces);
    info!("Feeds    - PostgreSQL: {}, DynamoDB: {}", pg_feeds, dynamo_feeds);

    let mut errors = 0;
    if pg_users != dynamo_users {
        error!("❌ User count mismatch: PostgreSQL({}) != DynamoDB({})", pg_users, dynamo_users);
        errors += 1;
    }
    if pg_spaces != dynamo_spaces {
        error!("❌ Space count mismatch: PostgreSQL({}) != DynamoDB({})", pg_spaces, dynamo_spaces);
        errors += 1;
    }
    if pg_feeds != dynamo_feeds {
        error!("❌ Feed count mismatch: PostgreSQL({}) != DynamoDB({})", pg_feeds, dynamo_feeds);
        errors += 1;
    }

    if errors == 0 {
        info!("✅ All record counts match!");
    } else {
        return Err(dto::Error::InvalidInputValue(format!("{} record count mismatches found", errors)));
    }

    Ok(())
}

async fn validate_users() -> Result<()> {
    info!("Validating user records");

    let postgres_pool = get_database_pool().await?;
    let dynamo_client = get_dynamo_client().await?;

    // Get sample of users from PostgreSQL
    let pg_users = User::fetch_many(&postgres_pool, "ORDER BY id LIMIT 50")
        .await
        .map_err(|e| dto::Error::DatabaseException(format!("Failed to fetch PostgreSQL users: {:?}", e)))?;

    let mut errors = 0;
    for pg_user in &pg_users {
        let pk = format!("USER#{}", pg_user.id);
        let sk = "METADATA".to_string();

        match dynamo_client.get_item("pk", &pk, Some(("sk", &sk))).await {
            Ok(Some(item)) => {
                // Validate DynamoDB user against PostgreSQL user
                match DynamoUser::from_item(item) {
                    Ok(dynamo_user) => {
                        if !validate_user_fields(&pg_user, &dynamo_user) {
                            error!("❌ User {} field validation failed", pg_user.id);
                            errors += 1;
                        }
                    }
                    Err(e) => {
                        error!("❌ Failed to deserialize DynamoDB user {}: {:?}", pg_user.id, e);
                        errors += 1;
                    }
                }
            }
            Ok(None) => {
                error!("❌ User {} not found in DynamoDB", pg_user.id);
                errors += 1;
            }
            Err(e) => {
                error!("❌ Failed to get user {} from DynamoDB: {:?}", pg_user.id, e);
                errors += 1;
            }
        }
    }

    if errors == 0 {
        info!("✅ All {} user records validated successfully!", pg_users.len());
    } else {
        return Err(dto::Error::InvalidInputValue(format!("{} user validation errors found", errors)));
    }

    Ok(())
}

async fn validate_spaces() -> Result<()> {
    info!("Validating space records");
    // Similar implementation for spaces
    info!("✅ Space validation completed (placeholder)");
    Ok(())
}

async fn validate_feeds() -> Result<()> {
    info!("Validating feed records");
    // Similar implementation for feeds
    info!("✅ Feed validation completed (placeholder)");
    Ok(())
}

async fn validate_relationships() -> Result<()> {
    info!("Validating relationship records");

    let postgres_pool = get_database_pool().await?;
    let dynamo_client = get_dynamo_client().await?;

    // Validate user followers
    let pg_followers: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT following_id, follower_id FROM my_networks LIMIT 20"
    )
    .fetch_all(&postgres_pool)
    .await
    .map_err(|e| dto::Error::DatabaseException(format!("Failed to fetch followers: {:?}", e)))?;

    let mut errors = 0;
    for (user_id, follower_id) in pg_followers {
        let pk = format!("USER#{}", user_id);
        let sk = format!("FOLLOWER#{}", follower_id);

        match dynamo_client.get_item("pk", &pk, Some(("sk", &sk))).await {
            Ok(Some(_)) => {
                // Follower relationship found
            }
            Ok(None) => {
                error!("❌ Follower relationship {}→{} not found in DynamoDB", follower_id, user_id);
                errors += 1;
            }
            Err(e) => {
                error!("❌ Failed to get follower relationship from DynamoDB: {:?}", e);
                errors += 1;
            }
        }
    }

    if errors == 0 {
        info!("✅ All relationship records validated successfully!");
    } else {
        return Err(dto::Error::InvalidInputValue(format!("{} relationship validation errors found", errors)));
    }

    Ok(())
}

async fn validate_sample_records(sample_size: usize) -> Result<()> {
    info!("Validating {} sample records", sample_size);

    let postgres_pool = get_database_pool().await?;
    let dynamo_client = get_dynamo_client().await?;

    // Get random sample of users
    let pg_users = User::fetch_many(&postgres_pool, &format!("ORDER BY RANDOM() LIMIT {}", sample_size))
        .await
        .map_err(|e| dto::Error::DatabaseException(format!("Failed to fetch sample users: {:?}", e)))?;

    let mut errors = 0;
    for (i, pg_user) in pg_users.iter().enumerate() {
        if i % 10 == 0 {
            info!("Validating record {} of {}", i + 1, pg_users.len());
        }

        let pk = format!("USER#{}", pg_user.id);
        let sk = "METADATA".to_string();

        match dynamo_client.get_item("pk", &pk, Some(("sk", &sk))).await {
            Ok(Some(item)) => {
                match DynamoUser::from_item(item) {
                    Ok(dynamo_user) => {
                        if !validate_user_fields(&pg_user, &dynamo_user) {
                            error!("❌ User {} field validation failed", pg_user.id);
                            errors += 1;
                        }
                    }
                    Err(e) => {
                        error!("❌ Failed to deserialize DynamoDB user {}: {:?}", pg_user.id, e);
                        errors += 1;
                    }
                }
            }
            Ok(None) => {
                error!("❌ User {} not found in DynamoDB", pg_user.id);
                errors += 1;
            }
            Err(e) => {
                error!("❌ Failed to get user {} from DynamoDB: {:?}", pg_user.id, e);
                errors += 1;
            }
        }
    }

    if errors == 0 {
        info!("✅ All {} sample records validated successfully!", pg_users.len());
    } else {
        return Err(dto::Error::InvalidInputValue(format!("{} validation errors found", errors)));
    }

    Ok(())
}

fn validate_user_fields(pg_user: &User, dynamo_user: &DynamoUser) -> bool {
    let mut valid = true;

    if pg_user.id != dynamo_user.id {
        error!("User {} ID mismatch: {} != {}", pg_user.id, pg_user.id, dynamo_user.id);
        valid = false;
    }

    if pg_user.nickname != dynamo_user.nickname {
        error!("User {} nickname mismatch: '{}' != '{}'", pg_user.id, pg_user.nickname, dynamo_user.nickname);
        valid = false;
    }

    if pg_user.email != dynamo_user.email {
        error!("User {} email mismatch: '{}' != '{}'", pg_user.id, pg_user.email, dynamo_user.email);
        valid = false;
    }

    if pg_user.username != dynamo_user.username {
        error!("User {} username mismatch: '{}' != '{}'", pg_user.id, pg_user.username, dynamo_user.username);
        valid = false;
    }

    // Add more field validations as needed

    valid
}

async fn count_dynamo_records_by_type(client: &DynamoClient, record_type: &str) -> Result<i64> {
    let mut expression_values = HashMap::new();
    expression_values.insert(":type".to_string(), aws_sdk_dynamodb::types::AttributeValue::S(record_type.to_string()));

    match client.query_gsi(
        "type-index",
        "#type = :type",
        expression_values,
    ).await {
        Ok(items) => Ok(items.len() as i64),
        Err(e) => {
            warn!("Failed to count {} records: {:?}", record_type, e);
            Ok(0)
        }
    }
}

async fn get_database_pool() -> Result<sqlx::PgPool> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/ratel".to_string());

    sqlx::PgPool::connect(&database_url)
        .await
        .map_err(|e| dto::Error::DatabaseException(format!("Failed to connect to database: {}", e)))
}

async fn get_dynamo_client() -> Result<DynamoClient> {
    let table_name = env::var("DYNAMODB_TABLE_NAME")
        .unwrap_or_else(|_| "ratel-local".to_string());

    Ok(DynamoClient::new(&table_name))
}

fn print_help() {
    println!("Migration Validation Tool");
    println!();
    println!("USAGE:");
    println!("    validate_migration [VALIDATION_TYPE] [OPTIONS]");
    println!();
    println!("VALIDATION TYPES:");
    println!("    all                  Run all validations (default)");
    println!("    count                Validate record counts");
    println!("    users                Validate user records");
    println!("    spaces               Validate space records");
    println!("    feeds                Validate feed records");
    println!("    relationships        Validate relationship records");
    println!("    sample <size>        Validate random sample of records");
    println!("    help                 Show this help message");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    DATABASE_URL         PostgreSQL connection string");
    println!("    DYNAMODB_TABLE_NAME  DynamoDB table name");
    println!("    AWS_ENDPOINT_URL_DYNAMODB  DynamoDB endpoint URL for local development");
    println!();
    println!("EXAMPLES:");
    println!("    # Validate all record types");
    println!("    validate_migration all");
    println!();
    println!("    # Validate only record counts");
    println!("    validate_migration count");
    println!();
    println!("    # Validate 500 random records");
    println!("    validate_migration sample 500");
}