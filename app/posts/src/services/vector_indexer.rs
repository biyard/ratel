use crate::models::Post;
use crate::*;
use common::utils::aws::{BedrockEmbeddingsClient, QdrantClient};

pub async fn index_post_async(
    qdrant: &QdrantClient,
    bedrock: &BedrockEmbeddingsClient,
    post: &Post,
) {
    if let Err(e) = index_post(qdrant, bedrock, post).await {
        tracing::error!("Failed to index post {:?}: {}", post.pk, e);
    }
}

pub fn delete_post_vector_async(qdrant: &QdrantClient, post_pk: &Partition) {
    let qdrant = qdrant.clone();
    let post_pk = post_pk.clone();

    dioxus::prelude::spawn(async move {
        let point_id = partition_to_uuid_string(&post_pk);
        if let Err(e) = qdrant.delete_point(point_id).await {
            tracing::error!("Failed to delete vector for post {:?}: {}", post_pk, e);
        }
    });
}

async fn index_post(
    qdrant: &QdrantClient,
    bedrock: &BedrockEmbeddingsClient,
    post: &Post,
) -> Result<()> {
    let plain_text = crate::utils::extract_plain_text(&post.html_contents);
    let embedding_input = format!("{} {}", post.title, plain_text).trim().to_string();

    if embedding_input.is_empty() {
        return Ok(());
    }

    let vector = bedrock.embed(&embedding_input).await?;

    let point_id = partition_to_uuid_string(&post.pk);

    let plain_text_preview: String = plain_text.chars().take(500).collect();

    let mut payload = serde_json::Map::new();
    payload.insert(
        "post_pk".to_string(),
        serde_json::to_value(&post.pk).unwrap_or_default(),
    );
    payload.insert(
        "user_pk".to_string(),
        serde_json::to_value(&post.user_pk).unwrap_or_default(),
    );
    payload.insert(
        "title".to_string(),
        serde_json::Value::String(post.title.clone()),
    );
    payload.insert(
        "status".to_string(),
        serde_json::to_value(&post.status).unwrap_or_default(),
    );
    payload.insert(
        "visibility".to_string(),
        serde_json::to_value(&post.visibility).unwrap_or_default(),
    );
    payload.insert(
        "post_type".to_string(),
        serde_json::to_value(&post.post_type).unwrap_or_default(),
    );
    payload.insert(
        "author_username".to_string(),
        serde_json::Value::String(post.author_username.clone()),
    );
    payload.insert(
        "author_display_name".to_string(),
        serde_json::Value::String(post.author_display_name.clone()),
    );
    payload.insert(
        "created_at".to_string(),
        serde_json::Value::Number(post.created_at.into()),
    );
    payload.insert(
        "updated_at".to_string(),
        serde_json::Value::Number(post.updated_at.into()),
    );
    payload.insert(
        "plain_text_preview".to_string(),
        serde_json::Value::String(plain_text_preview),
    );

    qdrant.upsert_point(point_id, vector, payload).await?;

    Ok(())
}

fn partition_to_uuid_string(pk: &Partition) -> String {
    let pk_str = format!("{:?}", pk);
    // Extract UUID part from partition key like Feed("uuid")
    if let Partition::Feed(uuid) = pk {
        uuid.to_string()
    } else {
        pk_str
    }
}
