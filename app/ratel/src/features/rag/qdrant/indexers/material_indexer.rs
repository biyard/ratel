use crate::common::Result;
use crate::features::rag::qdrant::payloads::MaterialPayload;
use crate::features::rag::qdrant::types::{QdrantIndexType, QdrantPayload};

use super::{tenant_id, upsert_point, delete_point};

/// Index reference material content into Qdrant.
pub async fn index_material(
    user_id: &str,
    space_id: &str,
    discussion_id: &str,
    material_id: &str,
    content: &str,
    file_name: &str,
) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let bedrock = config.bedrock_embeddings();
    let qdrant = config.qdrant();

    let vector = bedrock.embed(content).await?;

    let payload = MaterialPayload {
        r#type: QdrantIndexType::Material,
        tenant_id: tenant_id(),
        user_id: user_id.to_string(),
        space_id: space_id.to_string(),
        discussion_id: discussion_id.to_string(),
        material_id: material_id.to_string(),
        content: content.to_string(),
        file_name: file_name.to_string(),
    };

    upsert_point(qdrant, material_id, vector, payload.into_payload()).await
}

/// Remove material vectors from Qdrant.
pub async fn delete_material_vectors(material_id: &str) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    delete_point(qdrant, material_id).await
}
