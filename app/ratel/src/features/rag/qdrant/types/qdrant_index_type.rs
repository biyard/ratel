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
///
/// ```ignore
/// #[derive(Serialize)]
/// struct MyPayload { r#type: QdrantIndexType, content: String }
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
