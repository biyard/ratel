use crate::common::utils::aws::QdrantClient;
use serde::{Deserialize, Serialize};

/// Type discriminator for vectors stored in Qdrant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QdrantIndexType {
    Reply,
    Material,
}

impl std::fmt::Display for QdrantIndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QdrantIndexType::Reply => write!(f, "reply"),
            QdrantIndexType::Material => write!(f, "material"),
        }
    }
}

/// Marker trait for Qdrant payload structs.
///
/// Any struct that derives `Serialize` and implements this trait
/// automatically gets `into_payload()` via serde serialization.
/// All string-typed fields are serialized as JSON string values.
///
/// Usage:
/// ```ignore
/// #[derive(Serialize)]
/// struct MyPayload { ... }
/// impl QdrantPayload for MyPayload {}
///
/// let map = my_payload.into_payload();
/// ```
pub trait QdrantPayload: Serialize + Sized {
    fn into_payload(self) -> serde_json::Map<String, serde_json::Value> {
        match serde_json::to_value(&self) {
            Ok(serde_json::Value::Object(map)) => map,
            _ => serde_json::Map::new(),
        }
    }
}

/// Payload for indexing a discussion reply into Qdrant.
#[derive(Debug, Clone, Serialize)]
pub struct ReplyPayload {
    pub r#type: QdrantIndexType,
    pub content: String,
    pub author: String,
    pub author_pk: String,
}

impl QdrantPayload for ReplyPayload {}

/// Payload for indexing a reference material into Qdrant.
#[derive(Debug, Clone, Serialize)]
pub struct MaterialPayload {
    pub r#type: QdrantIndexType,
    pub content: String,
    pub file_name: String,
    pub material_id: String,
}

impl QdrantPayload for MaterialPayload {}

/// Create a per-discussion Qdrant client reusing the same URL/API key
/// from common Qdrant config, but with a discussion-scoped collection.
pub fn get_qdrant_client(space_id: &str, discussion_sk: &str) -> QdrantClient {
    let url = option_env!("QDRANT_URL")
        .unwrap_or("http://qdrant:6333")
        .to_string();
    let api_key = option_env!("QDRANT_API_KEY").map(|s| s.to_string());
    let prefix = option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local");
    let collection = format!("{}-aimod-{}-{}", prefix, space_id, discussion_sk);

    QdrantClient::new(url, collection, api_key)
}
