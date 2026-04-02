use crate::common::utils::aws::{BedrockEmbeddingsClient, QdrantClient};
use crate::common::Result;

fn collection_name(space_id: &str, discussion_sk: &str) -> String {
    let prefix =
        std::env::var("DYNAMO_TABLE_PREFIX").unwrap_or_else(|_| "ratel-local".to_string());
    format!("{}-aimod-{}-{}", prefix, space_id, discussion_sk)
}

pub fn get_qdrant_client(space_id: &str, discussion_sk: &str) -> QdrantClient {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://qdrant:6333".to_string());
    let api_key = std::env::var("QDRANT_API_KEY").ok();
    let collection = collection_name(space_id, discussion_sk);
    QdrantClient::new(url, collection, api_key)
}

pub async fn index_reply(
    qdrant: &QdrantClient,
    bedrock: &BedrockEmbeddingsClient,
    reply_id: &str,
    content: &str,
    author: &str,
) -> Result<()> {
    let vector = bedrock.embed(content).await?;
    let mut payload = serde_json::Map::new();
    payload.insert(
        "type".to_string(),
        serde_json::Value::String("reply".to_string()),
    );
    payload.insert(
        "content".to_string(),
        serde_json::Value::String(content.to_string()),
    );
    payload.insert(
        "author".to_string(),
        serde_json::Value::String(author.to_string()),
    );
    qdrant
        .upsert_point(reply_id.to_string(), vector, payload)
        .await
}

pub async fn index_material(
    qdrant: &QdrantClient,
    bedrock: &BedrockEmbeddingsClient,
    material_id: &str,
    content: &str,
    file_name: &str,
) -> Result<()> {
    let vector = bedrock.embed(content).await?;
    let mut payload = serde_json::Map::new();
    payload.insert(
        "type".to_string(),
        serde_json::Value::String("material".to_string()),
    );
    payload.insert(
        "content".to_string(),
        serde_json::Value::String(content.to_string()),
    );
    payload.insert(
        "file_name".to_string(),
        serde_json::Value::String(file_name.to_string()),
    );
    qdrant
        .upsert_point(material_id.to_string(), vector, payload)
        .await
}

pub async fn delete_material_vectors(
    qdrant: &QdrantClient,
    material_id: &str,
) -> Result<()> {
    qdrant.delete_point(material_id.to_string()).await
}
