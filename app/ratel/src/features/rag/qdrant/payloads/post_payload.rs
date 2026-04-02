use serde::Serialize;

use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

/// Payload for indexing a published post into Qdrant.
#[derive(Debug, Clone, Serialize)]
pub struct PostPayload {
    pub r#type: QdrantIndexType,
    pub post_pk: String,
    pub user_pk: String,
    pub title: String,
    pub status: String,
    pub visibility: String,
    pub post_type: String,
    pub author_username: String,
    pub author_display_name: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub plain_text_preview: String,
}

impl QdrantPayload for PostPayload {}
