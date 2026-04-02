use crate::common::Result;
use crate::features::rag::qdrant::payloads::MaterialPayload;
use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

use super::{discussion_collection, upsert_point, delete_point};

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
    let qdrant = config.qdrant();
    let collection = discussion_collection(space_id, discussion_sk);

    let vector = bedrock.embed(content).await?;

    let payload = MaterialPayload {
        r#type: QdrantIndexType::Material,
        content: content.to_string(),
        file_name: file_name.to_string(),
        material_id: material_id.to_string(),
    };

    upsert_point(qdrant, &collection, material_id, vector, payload.into_payload()).await
}

/// Remove material vectors from the discussion-scoped Qdrant collection.
pub async fn delete_material_vectors(
    space_id: &str,
    discussion_sk: &str,
    material_id: &str,
) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    let collection = discussion_collection(space_id, discussion_sk);
    delete_point(qdrant, &collection, material_id).await
}
