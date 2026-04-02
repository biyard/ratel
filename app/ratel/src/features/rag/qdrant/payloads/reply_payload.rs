use by_macros::QdrantEntity;
use serde::Serialize;

use crate::features::rag::qdrant::types::QdrantIndexType;

/// Payload for indexing a discussion reply into Qdrant.
#[derive(Debug, Clone, Serialize, QdrantEntity)]
#[qdrant(collection_name = "main")]
pub struct ReplyPayload {
    // Mandatory fields
    pub r#type: QdrantIndexType,
    pub tenant_id: String,
    pub user_id: String,
    pub space_id: String,
    // Reply-specific fields
    #[qdrant(id)]
    pub comment_id: String,
    pub discussion_id: String,
    pub content: String,
    pub author: String,
}

#[cfg(feature = "server")]
#[async_trait::async_trait]
impl crate::features::rag::qdrant::types::Embedding for ReplyPayload {
    async fn embed(&self) -> crate::common::Result<Vec<f32>> {
        let config = crate::common::CommonConfig::default();
        let bedrock = config.bedrock_embeddings();
        bedrock.embed(&self.content).await
    }
}

impl ReplyPayload {
    pub fn from_comment(
        comment: &crate::features::spaces::pages::actions::actions::discussion::SpacePostComment,
        tenant_id: String,
    ) -> Self {
        let space_pk_str = comment
            .space_pk
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_default();
        let space_id = space_pk_str
            .strip_prefix("SPACE#")
            .unwrap_or(&space_pk_str)
            .to_string();

        Self {
            r#type: QdrantIndexType::Reply,
            tenant_id,
            user_id: comment.author_pk.to_string(),
            space_id,
            comment_id: comment.sk.to_string(),
            discussion_id: comment.pk.to_string(),
            content: comment.content.clone(),
            author: comment.author_display_name.clone(),
        }
    }
}
