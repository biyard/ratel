use aws_lambda_events::dynamodb::EventRecord;
use aws_sdk_s3::Client as S3Client;
use lambda_runtime::Error as LambdaError;
use main_api::types::EntityType;
use serde_json::Value;
use std::str::FromStr;
use tokio::time::{Duration, sleep};
use tracing::error;

use super::s3::{get_object_if_exists, upload_object, upload_object_if_match};
use super::stream::StreamIdentifiers;

pub async fn update_space_aggregate(
    s3: &S3Client,
    bucket_name: &str,
    env: &str,
    record: &EventRecord,
    ids: &StreamIdentifiers,
) -> Result<(), LambdaError> {
    let Some(entity) = EntityType::from_str(ids.sk.as_str()).ok() else {
        return Ok(());
    };

    match entity {
        EntityType::SpaceCommon => {
            merge_record_into_array(
                s3,
                bucket_name,
                env,
                ids,
                record,
                "space_common.json",
                "SPACE_COMMON",
            )
            .await?;
        }
        EntityType::SpacePost(_) => {
            merge_record_into_array(
                s3,
                bucket_name,
                env,
                ids,
                record,
                "space_posts.json",
                "SPACE_POSTS",
            )
            .await?;
        }
        EntityType::SpacePostComment(_) => {
            merge_record_into_array(
                s3,
                bucket_name,
                env,
                ids,
                record,
                "space_post_comments.json",
                "SPACE_POST_COMMENTS",
            )
            .await?;
        }
        EntityType::SpacePoll(_) => {
            merge_record_into_array(
                s3,
                bucket_name,
                env,
                ids,
                record,
                "space_polls.json",
                "SPACE_POLLS",
            )
            .await?;
        }
        EntityType::SpacePollUserAnswer(_, _) => {
            merge_record_into_array(
                s3,
                bucket_name,
                env,
                ids,
                record,
                "space_poll_user_answers.json",
                "SPACE_POLL_USER_ANSWERS",
            )
            .await?;
        }
        EntityType::SpaceAnalyze => {
            upload_space_analyze(s3, bucket_name, env, ids, record).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn merge_record_into_array(
    s3: &S3Client,
    bucket_name: &str,
    env: &str,
    ids: &StreamIdentifiers,
    record: &EventRecord,
    filename: &str,
    sk_label: &str,
) -> Result<(), LambdaError> {
    let is_remove = record.event_name == "REMOVE";
    let item_value = if is_remove {
        None
    } else {
        let image = if !record.change.new_image.is_empty() {
            &record.change.new_image
        } else if !record.change.old_image.is_empty() {
            &record.change.old_image
        } else {
            return Ok(());
        };
        Some(stream_item_to_json(image)?)
    };

    let key = aggregate_key(env, ids.space_pk.as_str(), filename);
    let mut attempt = 0;
    let mut backoff = Duration::from_millis(50);

    loop {
        attempt += 1;
        let existing = get_object_if_exists(s3, bucket_name, &key).await?;
        let (etag, mut items) = match existing {
            Some(obj) => (obj.etag, parse_json_array(&obj.data)),
            None => (None, Vec::new()),
        };

        items.retain(|value| !matches_pk_sk(value, ids.pk.as_str(), ids.sk.as_str()));
        if let Some(item_value) = item_value.clone() {
            items.push(item_value);
        }

        let payload = serde_json::to_vec(&items).map_err(|e| {
            error!("failed to serialize aggregate payload: {e}");
            LambdaError::from("aggregate serialize failed")
        })?;

        let ok = upload_object_if_match(
            s3,
            bucket_name,
            &key,
            payload,
            "application/json",
            etag.as_deref(),
        )
        .await?;

        if ok {
            let metadata_key = format!("{}.metadata.json", key);
            let metadata_json =
                build_record_metadata(ids.space_pk.as_str(), ids.space_pk.as_str(), sk_label)?;
            upload_object(
                s3,
                bucket_name,
                &metadata_key,
                metadata_json,
                "application/json",
            )
            .await?;
            return Ok(());
        }

        if attempt >= 5 {
            error!("failed to update aggregate after retries: {}", key);
            return Err(LambdaError::from("aggregate update conflict"));
        }

        sleep(backoff).await;
        backoff = std::cmp::min(backoff * 2, Duration::from_millis(800));
    }
}

async fn upload_space_analyze(
    s3: &S3Client,
    bucket_name: &str,
    env: &str,
    ids: &StreamIdentifiers,
    record: &EventRecord,
) -> Result<(), LambdaError> {
    let image = if !record.change.new_image.is_empty() {
        &record.change.new_image
    } else if !record.change.old_image.is_empty() {
        &record.change.old_image
    } else {
        return Ok(());
    };

    let value = stream_item_to_json(image)?;
    let tf_idf = value
        .get("tf_idf")
        .cloned()
        .unwrap_or(Value::Array(Vec::new()));
    let lda = value
        .get("lda_topics")
        .cloned()
        .unwrap_or(Value::Array(Vec::new()));
    let network = value
        .get("network")
        .cloned()
        .unwrap_or(Value::Object(Default::default()));

    upload_aggregate_json(
        s3,
        bucket_name,
        env,
        ids.space_pk.as_str(),
        "space_analyze_tfidf.json",
        "SPACE_ANALYZE_TFIDF",
        &tf_idf,
    )
    .await?;
    upload_aggregate_json(
        s3,
        bucket_name,
        env,
        ids.space_pk.as_str(),
        "space_analyze_lda.json",
        "SPACE_ANALYZE_LDA",
        &lda,
    )
    .await?;
    upload_aggregate_json(
        s3,
        bucket_name,
        env,
        ids.space_pk.as_str(),
        "space_analyze_text_network.json",
        "SPACE_ANALYZE_TEXT_NETWORK",
        &network,
    )
    .await?;

    Ok(())
}

async fn upload_aggregate_json(
    s3: &S3Client,
    bucket_name: &str,
    env: &str,
    space_pk: &str,
    filename: &str,
    sk_label: &str,
    value: &Value,
) -> Result<(), LambdaError> {
    let key = aggregate_key(env, space_pk, filename);
    let payload = serde_json::to_vec(value).map_err(|e| {
        error!("failed to serialize aggregate payload: {e}");
        LambdaError::from("aggregate serialize failed")
    })?;

    upload_object(s3, bucket_name, &key, payload, "application/json").await?;

    let metadata_key = format!("{}.metadata.json", key);
    let metadata_json = build_record_metadata(space_pk, space_pk, sk_label)?;
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

fn aggregate_key(env: &str, space_pk: &str, filename: &str) -> String {
    format!("{}/spaces/{}/snapshots/{}", env, space_pk, filename)
}

fn stream_item_to_json(item: &serde_dynamo::Item) -> Result<Value, LambdaError> {
    serde_dynamo::from_item(item.clone()).map_err(|e| {
        error!("failed to convert stream item to json: {e}");
        LambdaError::from("stream item to json failed")
    })
}

fn parse_json_array(data: &[u8]) -> Vec<Value> {
    if let Ok(items) = serde_json::from_slice::<Vec<Value>>(data) {
        return items;
    }
    if let Ok(value) = serde_json::from_slice::<Value>(data) {
        if let Value::Array(items) = value {
            return items;
        }
        if value.is_object() {
            return vec![value];
        }
    }
    Vec::new()
}

fn matches_pk_sk(value: &Value, pk: &str, sk: &str) -> bool {
    let Some(obj) = value.as_object() else {
        return false;
    };
    let item_pk = obj.get("pk").and_then(|v| v.as_str());
    let item_sk = obj.get("sk").and_then(|v| v.as_str());
    matches!((item_pk, item_sk), (Some(p), Some(s)) if p == pk && s == sk)
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
