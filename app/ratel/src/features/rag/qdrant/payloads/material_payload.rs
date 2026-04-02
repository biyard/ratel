use serde::Serialize;

use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

/// Payload for indexing a reference material into Qdrant.
#[derive(Debug, Clone, Serialize)]
pub struct MaterialPayload {
    pub r#type: QdrantIndexType,
    pub content: String,
    pub file_name: String,
    pub material_id: String,
}

impl QdrantPayload for MaterialPayload {}
