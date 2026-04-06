use by_macros::QdrantEntity;
use serde::Serialize;

use crate::common::types::{FeedPartition, Partition, SpacePartition, UserOrTeam};
use crate::features::rag::qdrant::types::QdrantIndexType;

/// Payload for indexing a published post into Qdrant.
#[derive(Debug, Clone, Serialize, QdrantEntity)]
#[qdrant(collection_name = "main")]
pub struct PostPayload {
    // Mandatory fields
    pub r#type: QdrantIndexType,
    pub tenant_id: String,
    pub user_id: UserOrTeam,
    pub space_id: SpacePartition,
    // Post-specific fields
    #[qdrant(id)]
    pub post_id: FeedPartition,
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

#[cfg(feature = "server")]
#[async_trait::async_trait]
impl crate::features::rag::qdrant::types::Embedding for PostPayload {
    async fn embed(&self) -> crate::common::Result<Vec<f32>> {
        let config = crate::common::CommonConfig::default();
        let bedrock = config.bedrock_embeddings();
        let input = format!("{} {}", self.title, self.plain_text_preview);
        bedrock.embed(&input).await
    }
}

impl PostPayload {
    pub fn from_post(
        post: &crate::features::posts::models::Post,
        tenant_id: String,
        plain_text_preview: String,
    ) -> Self {
        let space_id = match &post.space_pk {
            Some(Partition::Space(inner)) => SpacePartition(inner.clone()),
            _ => SpacePartition::default(),
        };
        let post_id = match &post.pk {
            Partition::Feed(uuid) => FeedPartition(uuid.clone()),
            _ => FeedPartition(format!("{:?}", post.pk)),
        };

        Self {
            r#type: QdrantIndexType::Post,
            tenant_id,
            user_id: UserOrTeam::from(post.user_pk.clone()),
            space_id,
            post_id,
            title: post.title.clone(),
            status: serde_json::to_string(&post.status).unwrap_or_default(),
            visibility: serde_json::to_string(&post.visibility).unwrap_or_default(),
            post_type: serde_json::to_string(&post.post_type).unwrap_or_default(),
            author_username: post.author_username.clone(),
            author_display_name: post.author_display_name.clone(),
            created_at: post.created_at,
            updated_at: post.updated_at,
            plain_text_preview,
        }
    }
}
