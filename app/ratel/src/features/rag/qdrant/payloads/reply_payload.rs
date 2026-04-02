use serde::Serialize;

use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

/// Payload for indexing a discussion reply into Qdrant.
#[derive(Debug, Clone, Serialize)]
pub struct ReplyPayload {
    // Mandatory fields
    pub r#type: QdrantIndexType,
    pub tenant_id: String,
    pub user_id: String,
    pub space_id: String,
    // Reply-specific fields
    pub discussion_id: String,
    pub content: String,
    pub author: String,
}

impl QdrantPayload for ReplyPayload {}
