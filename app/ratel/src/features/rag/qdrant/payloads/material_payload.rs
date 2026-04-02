use by_macros::QdrantEntity;
use serde::Serialize;

use crate::features::rag::qdrant::types::QdrantIndexType;

/// Payload for indexing a reference material into Qdrant.
#[derive(Debug, Clone, Serialize, QdrantEntity)]
#[qdrant(collection_name = "main")]
pub struct MaterialPayload {
    // Mandatory fields
    pub r#type: QdrantIndexType,
    pub tenant_id: String,
    pub user_id: String,
    pub space_id: String,
    // Material-specific fields
    #[qdrant(id)]
    pub material_id: String,
    pub discussion_id: String,
    pub content: String,
    pub file_name: String,
}

#[cfg(feature = "server")]
#[async_trait::async_trait]
impl crate::features::rag::qdrant::types::Embedding for MaterialPayload {
    async fn embed(&self) -> crate::common::Result<Vec<f32>> {
        let config = crate::common::CommonConfig::default();
        let bedrock = config.bedrock_embeddings();
        bedrock.embed(&self.content).await
    }
}
