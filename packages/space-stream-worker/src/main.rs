mod config;
mod utils;

use aws_lambda_events::dynamodb::{Event as DynamoEvent, EventRecord};
use lambda_runtime::{Error as LambdaError, LambdaEvent};
#[cfg(not(feature = "local-run"))]
use lambda_runtime::{run, service_fn};
use serde_dynamo::{AttributeValue as StreamAttr, Item as StreamItem};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use aws_sdk_s3::Client as S3Client;
use utils::s3::{build_s3_client, upload_object};
use std::str::FromStr;

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

    let event_key = format!(
        "{}/spaces/{}/snapshots/{}_{}_snapshot.json",
        env,
        ids.space_pk,
        sanitize_key(ids.pk.as_str()),
        sanitize_key(ids.sk.as_str())
    );
    let payload = serde_json::to_vec(record).map_err(|e| {
        error!("failed to serialize stream record: {e}");
        LambdaError::from("event serialize failed")
    })?;

    upload_object(s3, bucket_name, &event_key, payload, "application/json").await?;

    let metadata_key = format!("{}.metadata.json", event_key);
    let metadata_json =
        build_record_metadata(ids.space_pk.as_str(), ids.pk.as_str(), ids.sk.as_str())?;
    upload_object(
        s3,
        bucket_name,
        &metadata_key,
        metadata_json,
        "application/json",
    )
    .await?;

    Ok(())
}

struct StreamIdentifiers {
    space_pk: String,
    pk: String,
    sk: String,
}

fn resolve_space_identifiers(
    record: &EventRecord,
) -> Option<StreamIdentifiers> {
    let new_pk = attr_string_stream(&record.change.new_image, "pk");
    let new_sk = attr_string_stream(&record.change.new_image, "sk");
    let key_pk = attr_string_stream(&record.change.keys, "pk");
    let key_sk = attr_string_stream(&record.change.keys, "sk");
    let old_pk = attr_string_stream(&record.change.old_image, "pk");
    let old_sk = attr_string_stream(&record.change.old_image, "sk");

    let pk = new_pk.or(key_pk).or(old_pk)?;
    let sk = new_sk
        .or(key_sk)
        .or(old_sk)
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let explicit_space_pk = attr_string_stream(&record.change.new_image, "space_pk")
        .or_else(|| attr_string_stream(&record.change.keys, "space_pk"))
        .or_else(|| attr_string_stream(&record.change.old_image, "space_pk"));

    let entity = main_api::types::EntityType::from_str(&sk).ok();
    if matches!(entity, Some(main_api::types::EntityType::SpacePost(_)))
        && explicit_space_pk.is_none()
    {
        return None;
    }

    let space_pk = explicit_space_pk
        .or_else(|| parse_space_pk_from_sk(&sk))
        .or_else(|| {
            if pk.starts_with("SPACE#") {
                Some(pk.clone())
            } else {
                None
            }
        });

    let space_pk = space_pk?;

    Some(StreamIdentifiers { space_pk, pk, sk })
}

fn build_record_metadata(space_pk: &str, pk: &str, sk: &str) -> Result<Vec<u8>, LambdaError> {
    let value = serde_json::json!({
        "metadataAttributes": {
            "space_pk": {
                "value": {
                    "type": "STRING",
                    "stringValue": space_pk
                },
                "includeForEmbedding": false
            },
            "pk": {
                "value": {
                    "type": "STRING",
                    "stringValue": pk
                },
                "includeForEmbedding": false
            },
            "sk": {
                "value": {
                    "type": "STRING",
                    "stringValue": sk
                },
                "includeForEmbedding": false
            }
        }
    });
    serde_json::to_vec(&value).map_err(|e| {
        error!("failed to serialize record metadata: {e}");
        LambdaError::from("record metadata serialize failed")
    })
}

fn is_space_related_pk(pk: &str) -> bool {
    pk.starts_with("SPACE#")
        || pk.starts_with("SPACE_POST#")
        || pk.starts_with("SPACE_POLL_USER_ANSWER#")
}

fn sanitize_key(value: &str) -> String {
    value.replace('/', "_")
}

fn attr_string_stream(image: &StreamItem, key: &str) -> Option<String> {
    image.get(key).and_then(|value| match value {
        StreamAttr::S(v) => Some(v.clone()),
        StreamAttr::N(v) => Some(v.clone()),
        _ => None,
    })
}

fn parse_space_pk_from_sk(sk: &str) -> Option<String> {
    const PREFIX: &str = "SPACE_POLL_USER_ANSWER#SPACE#";
    const POLL_MARKER: &str = "#POLL#";

    if let Some(rest) = sk.strip_prefix(PREFIX) {
        if let Some((space_id, _poll_part)) = rest.split_once(POLL_MARKER) {
            return Some(format!("SPACE#{}", space_id));
        }
    }
    None
}
