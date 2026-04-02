use crate::common::Result;
use crate::features::rag::qdrant::payloads::MaterialPayload;
use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

use super::get_discussion_qdrant_client;

/// Index reference material content into the discussion-scoped Qdrant collection.
pub async fn index_material(
    space_id: &str,
    discussion_sk: &str,
    material_id: &str,
    content: &str,
    file_name: &str,
) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let bedrock = config.bedrock_embeddings();
    let qdrant = get_discussion_qdrant_client(space_id, discussion_sk);

    let vector = bedrock.embed(content).await?;

    let payload = MaterialPayload {
        r#type: QdrantIndexType::Material,
        content: content.to_string(),
        file_name: file_name.to_string(),
        material_id: material_id.to_string(),
    };

    qdrant
        .upsert_point(material_id.to_string(), vector, payload.into_payload())
        .await
}

/// Remove material vectors from the discussion-scoped Qdrant collection.
pub async fn delete_material_vectors(
    space_id: &str,
    discussion_sk: &str,
    material_id: &str,
) -> Result<()> {
    let qdrant = get_discussion_qdrant_client(space_id, discussion_sk);
    qdrant.delete_point(material_id.to_string()).await
}
