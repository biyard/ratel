use serde::{Deserialize, Serialize};

/// Type discriminator for vectors stored in Qdrant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QdrantIndexType {
    Reply,
    Material,
    Post,
}

impl std::fmt::Display for QdrantIndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QdrantIndexType::Reply => write!(f, "reply"),
            QdrantIndexType::Material => write!(f, "material"),
            QdrantIndexType::Post => write!(f, "post"),
        }
    }
}

/// Trait for Qdrant entities that can produce embedding vectors.
///
/// Implement this on any `#[derive(QdrantEntity)]` struct to enable
/// `upsert_points()`.
#[cfg(feature = "server")]
#[async_trait::async_trait]
pub trait Embedding {
    async fn embed(&self) -> crate::common::Result<Vec<f32>>;
}

/// Convert a `serde_json::Value` into a `qdrant_client::qdrant::Value`.
///
/// Used by the `QdrantEntity` derive macro to build payloads.
#[cfg(feature = "server")]
pub fn json_to_qdrant_value(v: serde_json::Value) -> qdrant_client::qdrant::Value {
    match v {
        serde_json::Value::String(s) => qdrant_client::qdrant::Value::from(s),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                qdrant_client::qdrant::Value::from(i)
            } else if let Some(f) = n.as_f64() {
                qdrant_client::qdrant::Value::from(f)
            } else {
                qdrant_client::qdrant::Value::from(n.to_string())
            }
        }
        serde_json::Value::Bool(b) => qdrant_client::qdrant::Value::from(b),
        serde_json::Value::Null => qdrant_client::qdrant::Value::from(""),
        serde_json::Value::Array(arr) => {
            let list: Vec<qdrant_client::qdrant::Value> =
                arr.into_iter().map(json_to_qdrant_value).collect();
            qdrant_client::qdrant::Value::from(list)
        }
        serde_json::Value::Object(map) => {
            let inner: std::collections::HashMap<String, qdrant_client::qdrant::Value> = map
                .into_iter()
                .map(|(k, v)| (k, json_to_qdrant_value(v)))
                .collect();
            qdrant_client::qdrant::Value {
                kind: Some(qdrant_client::qdrant::value::Kind::StructValue(
                    qdrant_client::qdrant::Struct { fields: inner },
                )),
            }
        }
    }
}
