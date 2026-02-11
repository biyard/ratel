mod config;
mod utils;

use aws_lambda_events::dynamodb::Event as DynamoEvent;
use lambda_runtime::{Error as LambdaError, LambdaEvent};
#[cfg(not(feature = "local-run"))]
use lambda_runtime::{run, service_fn};
use serde::Serialize;
use serde_dynamo::{AttributeValue as StreamAttr, Item as StreamItem};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_s3::Client as S3Client;
use main_api::features::spaces::{
    analyzes::SpaceAnalyze,
    boards::models::space_post::SpacePost,
    boards::models::space_post_comment::SpacePostComment,
    models::SpaceCommon,
    panels::SpacePanelParticipant,
    polls::{Poll, PollQuestion, PollUserAnswer},
};
use main_api::types::{EntityType, Partition};
use std::str::FromStr;
use utils::dynamo::{
    build_dynamo_client, fetch_all_panel_participants, fetch_all_polls, fetch_all_posts,
    fetch_poll_user_answers, fetch_post_comments, map_model_error,
};
use utils::s3::{build_s3_client, upload_object};

#[derive(Debug, Serialize)]
struct SpaceSnapshot {
    space_pk: String,
    status: String,
    updated_at: Option<i64>,
    captured_at: i64,
    space: Option<SpaceCommon>,
    analyze: Option<SpaceAnalyze>,
    polls: Vec<Poll>,
    poll_question: Option<PollQuestion>,
    poll_user_answers: Vec<PollUserAnswer>,
    panel_participants: Vec<SpacePanelParticipant>,
    posts: Vec<SpacePost>,
    post_comments: Vec<SpacePostComment>,
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
    let dynamo = build_dynamo_client(conf.dynamo_endpoint).await?;

    for record in event.records {
        if record.event_name != "MODIFY" {
            continue;
        }
        let change = record.change;
        let new_image = &change.new_image;
        let new_status = attr_string_stream(new_image, "status");
        if !matches!(new_status.as_deref(), Some("Finished") | Some("FINISHED")) {
            continue;
        }

        let old_status = attr_string_stream(&change.old_image, "status");
        if matches!(old_status.as_deref(), Some("Finished") | Some("FINISHED")) {
            continue;
        }

        handle_finish_record(
            &s3,
            &dynamo,
            bucket_name,
            conf.env,
            new_image,
            new_status.as_deref(),
        )
        .await?;
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
    dynamo: &DynamoClient,
    bucket_name: &str,
    env: &str,
    new_image: &StreamItem,
    status: Option<&str>,
) -> Result<(), LambdaError> {
    let space_pk = match attr_string_stream(new_image, "pk") {
        Some(pk) => pk,
        None => {
            error!("stream record missing pk");
            return Ok(());
        }
    };

    let updated_at = attr_i64_stream(new_image, "updated_at");
    let captured_at = chrono::Utc::now().timestamp_millis();
    let status = status.unwrap_or("Finished").to_string();

    let snapshot = build_snapshot(dynamo, &space_pk, status, updated_at, captured_at).await?;

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

    let metadata_key = format!("{}.metadata.json", key);
    let metadata_json = build_snapshot_metadata(space_pk.as_str())?;
    if let Err(err) = upload_object(
        s3,
        bucket_name,
        &metadata_key,
        metadata_json,
        "application/json",
    )
    .await
    {
        error!("failed to upload snapshot metadata: {err:?}");
        return Err(LambdaError::from("snapshot metadata upload failed"));
    }

    info!("snapshot uploaded for space_pk={}", space_pk);
    Ok(())
}

fn build_snapshot_metadata(space_pk: &str) -> Result<Vec<u8>, LambdaError> {
    let value = serde_json::json!({
        "metadataAttributes": {
            "space_pk": {
                "value": {
                    "type": "STRING",
                    "stringValue": space_pk
                },
                "includeForEmbedding": false
            }
        }
    });
    serde_json::to_vec(&value).map_err(|e| {
        error!("failed to serialize snapshot metadata: {e}");
        LambdaError::from("snapshot metadata serialize failed")
    })
}

fn attr_string_stream(image: &StreamItem, key: &str) -> Option<String> {
    image.get(key).and_then(|value| match value {
        StreamAttr::S(v) => Some(v.clone()),
        StreamAttr::N(v) => Some(v.clone()),
        _ => None,
    })
}

fn attr_i64_stream(image: &StreamItem, key: &str) -> Option<i64> {
    image.get(key).and_then(|value| match value {
        StreamAttr::N(v) => v.parse::<i64>().ok(),
        StreamAttr::S(v) => v.parse::<i64>().ok(),
        _ => None,
    })
}

async fn build_snapshot(
    dynamo: &DynamoClient,
    space_pk_raw: &str,
    status: String,
    updated_at: Option<i64>,
    captured_at: i64,
) -> Result<SpaceSnapshot, LambdaError> {
    let space_pk = Partition::from_str(space_pk_raw).map_err(|e| {
        error!("invalid space_pk {}: {e}", space_pk_raw);
        LambdaError::from("invalid space_pk")
    })?;

    let (space, analyze, poll_question, polls, panel_participants, posts) = tokio::try_join!(
        async {
            SpaceCommon::get(dynamo, space_pk.clone(), Some(EntityType::SpaceCommon))
                .await
                .map_err(|e| map_model_error("space_common", e))
        },
        async {
            SpaceAnalyze::get(dynamo, space_pk.clone(), Some(EntityType::SpaceAnalyze))
                .await
                .map_err(|e| map_model_error("space_analyze", e))
        },
        async {
            PollQuestion::get(
                dynamo,
                space_pk.clone(),
                Some(EntityType::SpacePollQuestion),
            )
            .await
            .map_err(|e| map_model_error("poll_question", e))
        },
        fetch_all_polls(dynamo, &space_pk),
        fetch_all_panel_participants(dynamo, &space_pk),
        fetch_all_posts(dynamo, &space_pk),
    )?;

    let (poll_user_answers, post_comments) = tokio::try_join!(
        fetch_poll_user_answers(dynamo, &space_pk, &polls),
        fetch_post_comments(dynamo, &posts),
    )?;

    Ok(SpaceSnapshot {
        space_pk: space_pk_raw.to_string(),
        status,
        updated_at,
        captured_at,
        space,
        analyze,
        polls,
        poll_question,
        poll_user_answers,
        panel_participants,
        posts,
        post_comments,
    })
}
