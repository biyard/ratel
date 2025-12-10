use std::sync::Arc;

use aws_lambda_events::{encodings, sqs::SqsEvent};
use dto::{
    Error,
    by_types::DatabaseConfig,
    sqlx::{PgPool, postgres::PgPoolOptions},
};
use lambda_runtime::{LambdaEvent, run, service_fn};
mod config;
mod s3_config;
mod utils;
use tracing_subscriber::EnvFilter;
use utils::s3_client::S3Client;

use crate::utils::watermark::process_watermark_async;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from(option_env!("RUST_LOG").unwrap_or("info")))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    let conf = config::get();
    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        let res = PgPoolOptions::new()
            .max_connections(pool_size)
            .connect_lazy(url);

        match res {
            Ok(pool) => {
                tracing::info!("Postgres pool created successfully");
                pool
            }
            Err(e) => {
                tracing::error!("Failed to create Postgres pool: {:?}", e);
                return Err(e.into());
            }
        }
    } else {
        panic!("Database is not initialized. Call init() first.");
    };
    let s3_client = S3Client::new().await?;
    let handler_pool = pool.clone();
    let handler_s3 = s3_client.clone();
    let handler = move |event| {
        let pool = handler_pool.clone();
        let s3 = handler_s3.clone();
        async move { function_handler(event, pool, s3).await }
    };
    Ok(run(service_fn(handler)).await.map_err(|e| {
        tracing::error!("Lambda function failed: {}", e);
        Error::ServerError("Lambda function execution failed".to_string())
    })?)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WatermarkTask {
    pub artwork_id: i64,
    pub original_url: String,
}

async fn function_handler(
    event: LambdaEvent<SqsEvent>,
    pool: PgPool,
    s3_client: Arc<S3Client>,
) -> Result<(), encodings::Error> {
    let (event, context) = event.into_parts();
    let len = event.records.len();
    for record in event.records {
        let body = record.body.ok_or("Failed to get message body")?;
        let message: WatermarkTask = serde_json::from_str(&body)?;
        let _ = process_watermark_async(
            &pool,
            s3_client.clone(),
            message.artwork_id,
            message.original_url,
        )
        .await;
    }
    tracing::info!("Processing request {}", context.request_id);
    tracing::info!("Processed {} records", len);
    Ok(())
}
