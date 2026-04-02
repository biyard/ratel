use crate::common::{Result, Error};
use crate::common::types::Partition;
use crate::features::posts::models::Post;
use crate::features::posts::types::PostStatus;
use crate::features::rag::qdrant::payloads::PostPayload;
use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

/// Index a published Post into the global posts Qdrant collection.
///
/// Called from DynamoStream when a Post is modified with status=PUBLISHED.
/// Acquires QdrantClient and BedrockEmbeddingsClient from common config.
pub async fn index_post(post: Post) -> Result<()> {
    if post.status != PostStatus::Published {
        return Ok(());
    }

    let config = crate::common::CommonConfig::default();
    let bedrock = config.bedrock_embeddings();
    let qdrant = config.qdrant();

    let plain_text = crate::features::posts::utils::extract_plain_text(&post.html_contents);
    let embedding_input = format!("{} {}", post.title, plain_text).trim().to_string();

    if embedding_input.is_empty() {
        return Ok(());
    }

    let vector = bedrock.embed(&embedding_input).await?;
    let point_id = partition_to_uuid_string(&post.pk);
    let plain_text_preview: String = plain_text.chars().take(500).collect();

    let payload = PostPayload {
        r#type: QdrantIndexType::Post,
        post_pk: post.pk.to_string(),
        user_pk: post.user_pk.to_string(),
        title: post.title,
        status: serde_json::to_string(&post.status).unwrap_or_default(),
        visibility: serde_json::to_string(&post.visibility).unwrap_or_default(),
        post_type: serde_json::to_string(&post.post_type).unwrap_or_default(),
        author_username: post.author_username,
        author_display_name: post.author_display_name,
        created_at: post.created_at,
        updated_at: post.updated_at,
        plain_text_preview,
    };

    qdrant
        .upsert_point(point_id, vector, payload.into_payload())
        .await
}

/// Delete a Post's vector from the global posts Qdrant collection.
///
/// Called from DynamoStream when a published Post is deleted.
pub async fn delete_post_index(post: Post) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();

    let point_id = partition_to_uuid_string(&post.pk);
    qdrant.delete_point(point_id).await
}

fn partition_to_uuid_string(pk: &Partition) -> String {
    if let Partition::Feed(uuid) = pk {
        uuid.to_string()
    } else {
        format!("{:?}", pk)
    }
}
