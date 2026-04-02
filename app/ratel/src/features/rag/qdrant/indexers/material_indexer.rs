use crate::common::Result;
use crate::common::types::{SpacePartition, UserOrTeam};
use crate::features::rag::qdrant::payloads::MaterialPayload;
use crate::features::rag::qdrant::types::QdrantIndexType;

/// Index reference material content into Qdrant.
pub async fn index_material(
    user_id: UserOrTeam,
    space_id: SpacePartition,
    discussion_id: &str,
    material_id: &str,
    content: &str,
    file_name: &str,
) -> Result<()> {
    let tenant_id = super::tenant_id();
    let payload = MaterialPayload {
        r#type: QdrantIndexType::Material,
        tenant_id,
        user_id,
        space_id,
        discussion_id: discussion_id.to_string(),
        material_id: material_id.to_string(),
        content: content.to_string(),
        file_name: file_name.to_string(),
    };

    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    payload.upsert_points(qdrant).await?;
    Ok(())
}

/// Remove material vectors from Qdrant.
pub async fn delete_material_vectors(material_id: &str) -> Result<()> {
    let config = crate::common::CommonConfig::default();
    let qdrant = config.qdrant();
    MaterialPayload::delete_points(qdrant, material_id).await?;
    Ok(())
}
