use bdk::prelude::*;
use dto::Result;
use main_api::etl::PostgresToDynamoMigrator;
use std::env;
use tracing::{info, error};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting PostgreSQL to DynamoDB migration");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("migrate");

    match command {
        "migrate" => run_full_migration().await,
        "resume" => {
            let migration_id = args.get(2).ok_or_else(|| {
                dto::Error::Unknown("Migration ID required for resume".to_string())
            })?;
            resume_migration(migration_id).await
        }
        "status" => {
            let migration_id = args.get(2).ok_or_else(|| {
                dto::Error::Unknown("Migration ID required for status check".to_string())
            })?;
            check_migration_status(migration_id).await
        }
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
            std::process::exit(1);
        }
    }
}

async fn run_full_migration() -> Result<()> {
    info!("Starting full migration from PostgreSQL to DynamoDB");

    // Get database pool
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/ratel".to_string());
    
    let pool = get_database_pool(&database_url).await?;

    // Get DynamoDB table name
    let table_name = env::var("DYNAMODB_TABLE_NAME")
        .unwrap_or_else(|_| "ratel-local".to_string());

    info!("Using database: {}", database_url);
    info!("Target DynamoDB table: {}", table_name);

    // Create migrator
    let mut migrator = PostgresToDynamoMigrator::new(&table_name, pool);

    // Set up graceful shutdown
    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        info!("Received shutdown signal, stopping migration...");
    };

    // Run migration with graceful shutdown
    tokio::select! {
        result = migrator.migrate_all_tables() => {
            match result {
                Ok(_) => {
                    info!("Migration completed successfully!");
                    let state = migrator.get_migration_state();
                    info!("Final statistics:");
                    info!("  Migration ID: {}", state.migration_id);
                    info!("  Total records: {}", state.total_records);
                    info!("  Migrated records: {}", state.migrated_records);
                    info!("  Progress: {:.2}%", state.progress_percentage());
                    info!("  Errors: {}", state.error_count);
                    
                    if state.error_count > 0 {
                        info!("  Last error: {:?}", state.last_error);
                    }
                    
                    Ok(())
                }
                Err(e) => {
                    error!("Migration failed: {:?}", e);
                    
                    let state = migrator.get_migration_state();
                    error!("Migration state at failure:");
                    error!("  Migration ID: {}", state.migration_id);
                    error!("  Progress: {:.2}%", state.progress_percentage());
                    error!("  Migrated records: {}/{}", state.migrated_records, state.total_records);
                    error!("  Errors: {}", state.error_count);
                    
                    // Save state for potential resume
                    save_migration_state(state).await?;
                    error!("Migration state saved. You can resume with: migrate_to_dynamo resume {}", state.migration_id);
                    
                    Err(e)
                }
            }
        }
        _ = shutdown_signal => {
            info!("Graceful shutdown initiated");
            let state = migrator.get_migration_state();
            save_migration_state(state).await?;
            info!("Migration state saved. You can resume with: migrate_to_dynamo resume {}", state.migration_id);
            Ok(())
        }
    }
}

async fn resume_migration(migration_id: &str) -> Result<()> {
    info!("Resuming migration: {}", migration_id);

    // Load migration state
    let state = load_migration_state(migration_id).await?;
    
    info!("Loaded migration state:");
    info!("  Progress: {:.2}%", state.progress_percentage());
    info!("  Migrated records: {}/{}", state.migrated_records, state.total_records);
    info!("  Status: {:?}", state.status);

    // Get database pool
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/ratel".to_string());
    
    let pool = get_database_pool(&database_url).await?;

    // Get DynamoDB table name
    let table_name = env::var("DYNAMODB_TABLE_NAME")
        .unwrap_or_else(|_| "ratel-local".to_string());

    // Create migrator and resume
    let mut migrator = PostgresToDynamoMigrator::new(&table_name, pool);
    
    migrator.resume_migration(state).await?;
    
    info!("Migration resumed and completed successfully!");
    Ok(())
}

async fn check_migration_status(migration_id: &str) -> Result<()> {
    info!("Checking migration status: {}", migration_id);

    let state = load_migration_state(migration_id).await?;
    
    println!("Migration Status Report");
    println!("======================");
    println!("Migration ID: {}", state.migration_id);
    println!("Status: {:?}", state.status);
    println!("Started at: {}", chrono::DateTime::from_timestamp(state.started_at, 0).unwrap());
    
    if let Some(completed_at) = state.completed_at {
        println!("Completed at: {}", chrono::DateTime::from_timestamp(completed_at, 0).unwrap());
    }
    
    println!("Progress: {:.2}%", state.progress_percentage());
    println!("Total records: {}", state.total_records);
    println!("Migrated records: {}", state.migrated_records);
    println!("Errors: {}", state.error_count);
    
    if let Some(ref last_error) = state.last_error {
        println!("Last error: {}", last_error);
    }
    
    println!("\nTable Status:");
    println!("=============");
    for (table_name, table_state) in &state.table_states {
        println!("  {}: {:?} ({}/{})", 
                 table_name, 
                 table_state.status,
                 table_state.migrated_records,
                 table_state.total_records);
    }

    Ok(())
}

async fn save_migration_state(state: &main_api::etl::MigrationState) -> Result<()> {
    let state_json = serde_json::to_string_pretty(state)
        .map_err(|e| dto::Error::DynamoDbSerializationError(format!("Failed to serialize state: {}", e)))?;
    
    let filename = format!("migration_state_{}.json", state.migration_id);
    tokio::fs::write(&filename, state_json)
        .await
        .map_err(|e| dto::Error::ServerError(format!("Failed to save state file: {}", e)))?;
    
    info!("Migration state saved to: {}", filename);
    Ok(())
}

async fn load_migration_state(migration_id: &str) -> Result<main_api::etl::MigrationState> {
    let filename = format!("migration_state_{}.json", migration_id);
    let state_json = tokio::fs::read_to_string(&filename)
        .await
        .map_err(|e| dto::Error::ServerError(format!("Failed to load state file {}: {}", filename, e)))?;
    
    let state: main_api::etl::MigrationState = serde_json::from_str(&state_json)
        .map_err(|e| dto::Error::JsonDeserializeError(format!("Failed to deserialize state: {}", e)))?;
    
    Ok(state)
}

async fn get_database_pool(database_url: &str) -> Result<sqlx::PgPool> {
    info!("Connecting to database: {}", database_url);
    
    let pool = sqlx::PgPool::connect(database_url)
        .await
        .map_err(|e| dto::Error::DatabaseException(format!("Failed to connect to database: {}", e)))?;
    
    info!("Database connection established");
    Ok(pool)
}

fn print_help() {
    println!("PostgreSQL to DynamoDB Migration Tool");
    println!();
    println!("USAGE:");
    println!("    migrate_to_dynamo [COMMAND] [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    migrate              Run full migration (default)");
    println!("    resume <migration_id> Resume a paused/failed migration");
    println!("    status <migration_id> Check status of a migration");
    println!("    help                 Show this help message");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    DATABASE_URL         PostgreSQL connection string");
    println!("                        (default: postgresql://localhost:5432/ratel)");
    println!("    DYNAMODB_TABLE_NAME  Target DynamoDB table name");
    println!("                        (default: ratel-local)");
    println!("    AWS_ENDPOINT_URL_DYNAMODB  DynamoDB endpoint URL for local development");
    println!();
    println!("EXAMPLES:");
    println!("    # Run full migration");
    println!("    migrate_to_dynamo migrate");
    println!();
    println!("    # Resume migration");
    println!("    migrate_to_dynamo resume migration_1234567890");
    println!();
    println!("    # Check migration status");
    println!("    migrate_to_dynamo status migration_1234567890");
    println!();
    println!("    # Use custom database URL");
    println!("    DATABASE_URL=postgresql://user:pass@host:5432/db migrate_to_dynamo");
}