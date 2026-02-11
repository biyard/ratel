mod config;

use aws_lambda_events::dynamodb::Event as DynamoEvent;
use lambda_runtime::{Error as LambdaError, LambdaEvent};
use serde::Serialize;
use serde_dynamo::{AttributeValue, Item};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};

#[derive(Debug, Serialize)]
struct SpaceFinishSnapshot<'a> {
    space_pk: &'a str,
    status: &'a str,
    updated_at: Option<i64>,
    captured_at: i64,
}

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
        if record.event_name != "MODIFY" {
            continue;
        }
        let change = record.change;
        let new_image = &change.new_image;
        let new_status = attr_string(new_image, "status");
        if !matches!(new_status.as_deref(), Some("Finished") | Some("FINISHED")) {
            continue;
        }

        let old_status = attr_string(&change.old_image, "status");

        if matches!(old_status.as_deref(), Some("Finished") | Some("FINISHED")) {
            continue;
        }

        handle_finish_record(&s3, bucket_name, conf.env, new_image, new_status.as_deref()).await?;
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

async fn handle_finish_record(
    s3: &S3Client,
    bucket_name: &str,
    env: &str,
    new_image: &Item,
    status: Option<&str>,
) -> Result<(), LambdaError> {
    let space_pk = match attr_string(new_image, "pk") {
        Some(pk) => pk,
        None => {
            error!("stream record missing pk");
            return Ok(());
        }
    };

    let updated_at = attr_i64(new_image, "updated_at");
    let captured_at = chrono::Utc::now().timestamp_millis();

    let snapshot = SpaceFinishSnapshot {
        space_pk: &space_pk,
        status: status.unwrap_or("Finished"),
        updated_at,
        captured_at,
    };

    let snapshot_json = serde_json::to_vec(&snapshot).map_err(|e| {
        error!("failed to serialize snapshot: {e}");
        LambdaError::from("snapshot serialize failed")
    })?;

    let key = format!("{}/spaces/{}/snapshots/snapshot.json", env, space_pk);
    if let Err(err) = upload_object(s3, bucket_name, &key, snapshot_json, "application/json").await
    {
        error!("failed to upload snapshot: {err:?}");
        return Err(LambdaError::from("snapshot upload failed"));
    }

    info!("snapshot uploaded for space_pk={}", space_pk);
    Ok(())
}

async fn build_s3_client() -> Result<S3Client, LambdaError> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(S3Client::new(&aws_config))
}

async fn upload_object(
    s3: &S3Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<(), LambdaError> {
    s3.put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| {
            error!("failed to upload snapshot object: {e}");
            LambdaError::from("s3 put_object failed")
        })?;
    Ok(())
}

fn attr_string(image: &Item, key: &str) -> Option<String> {
    image.get(key).and_then(|value| match value {
        AttributeValue::S(v) => Some(v.clone()),
        AttributeValue::N(v) => Some(v.clone()),
        _ => None,
    })
}

fn attr_i64(image: &Item, key: &str) -> Option<i64> {
    image.get(key).and_then(|value| match value {
        AttributeValue::N(v) => v.parse::<i64>().ok(),
        AttributeValue::S(v) => v.parse::<i64>().ok(),
        _ => None,
    })
}
