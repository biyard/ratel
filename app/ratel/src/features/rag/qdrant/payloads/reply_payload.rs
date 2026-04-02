use serde::Serialize;

use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

/// Payload for indexing a discussion reply into Qdrant.
#[derive(Debug, Clone, Serialize)]
pub struct ReplyPayload {
    pub r#type: QdrantIndexType,
    pub content: String,
    pub author: String,
    pub author_pk: String,
}

impl QdrantPayload for ReplyPayload {}
