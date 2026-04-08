use by_macros::QdrantEntity;
use serde::Serialize;

use crate::common::types::{Partition, SpacePartition, UserOrTeam};
use crate::features::rag::qdrant::types::QdrantIndexType;
use crate::types::*;

/// Payload for indexing a discussion reply into Qdrant.
#[derive(Debug, Clone, Serialize, QdrantEntity)]
#[qdrant(collection_name = "main")]
pub struct ReplyPayload {
    // Mandatory fields
    pub r#type: QdrantIndexType,
    pub tenant_id: String,
    pub user_id: UserOrTeam,
    pub space_id: SpacePartition,
    // Reply-specific fields
    #[qdrant(id)]
    pub comment_id: SpacePostCommentEntityType,
    pub discussion_id: SpacePostPartition,
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
        let space_id = comment.space_pk.clone().unwrap().into();

        Self {
            r#type: QdrantIndexType::Reply,
            tenant_id,
            user_id: UserOrTeam::from(comment.author_pk.clone()),
            space_id,
            comment_id: comment.sk.clone().into(),
            discussion_id: comment.pk.clone().into(),
            content: comment.content.clone(),
            author: comment.author_display_name.clone(),
        }
    }
}
