use serde::Serialize;

use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

/// Payload for indexing a reference material into Qdrant.
#[derive(Debug, Clone, Serialize)]
pub struct MaterialPayload {
    // Mandatory fields
    pub r#type: QdrantIndexType,
    pub tenant_id: String,
    pub user_id: String,
    pub space_id: String,
    // Material-specific fields
    pub discussion_id: String,
    pub material_id: String,
    pub content: String,
    pub file_name: String,
}

impl QdrantPayload for MaterialPayload {}
