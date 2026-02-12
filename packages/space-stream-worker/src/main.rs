mod config;
mod utils;

use aws_lambda_events::dynamodb::{Event as DynamoEvent, EventRecord};
use aws_sdk_s3::Client as S3Client;
use lambda_runtime::{Error as LambdaError, LambdaEvent};
#[cfg(not(feature = "local-run"))]
use lambda_runtime::{run, service_fn};
use tracing::info;
use tracing_subscriber::EnvFilter;
use utils::aggregate::update_space_aggregate;
use utils::s3::build_s3_client;
use utils::stream::{is_space_related_pk, resolve_space_identifiers};

#[cfg(not(feature = "local-run"))]
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    init_tracing();
    run(service_fn(handler)).await
}

#[cfg(feature = "local-run")]
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    use lambda_runtime::Context;

    init_tracing();
    let payload = load_event_payload()?;
    let ctx = Context::default();
    handler(LambdaEvent::new(payload, ctx)).await
}

async fn handler(event: LambdaEvent<DynamoEvent>) -> Result<(), LambdaError> {
    let (event, ctx) = event.into_parts();
    info!("space-stream-worker invoked: request_id={}", ctx.request_id);

    let conf = config::get();
    let bucket_name = conf.private_bucket_name;
    let s3 = build_s3_client().await?;

    for record in event.records {
        persist_event_record(&s3, bucket_name, conf.env, &record).await?;
    }

    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();
}

#[cfg(feature = "local-run")]
fn load_event_payload() -> Result<DynamoEvent, LambdaError> {
    let path = {
        let cwd = std::env::current_dir().ok();
        let cwd_candidate = cwd
            .as_ref()
            .map(|dir| dir.join("fixtures/space-finish-event.json"));
        if let Some(candidate) = cwd_candidate.as_ref().filter(|p| p.exists()) {
            candidate.to_string_lossy().to_string()
        } else {
            format!(
                "{}/fixtures/space-finish-event.json",
                env!("CARGO_MANIFEST_DIR")
            )
        }
    };
    let data = std::fs::read(&path).map_err(|e| {
        error!("failed to read event file {}: {e}", path);
        LambdaError::from("event file read failed")
    })?;
    serde_json::from_slice(&data).map_err(|e| {
        error!("failed to parse event json: {e}");
        LambdaError::from("event json parse failed")
    })
}

async fn persist_event_record(
    s3: &S3Client,
    bucket_name: &str,
    env: &str,
    record: &EventRecord,
) -> Result<(), LambdaError> {
    let Some(ids) = resolve_space_identifiers(record) else {
        return Ok(());
    };

    if !is_space_related_pk(ids.pk.as_str()) {
        return Ok(());
    }

    update_space_aggregate(s3, bucket_name, env, record, &ids).await?;

    Ok(())
}
