use crate::features::ai_moderator::*;

#[delete("/api/spaces/{space_id}/discussions/{discussion_sk}/ai-moderator/materials/{material_id}", role: SpaceUserRole)]
pub async fn delete_material(
    space_id: SpacePartition,
    discussion_sk: String,
    material_id: String,
) -> Result<()> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk = CompositePartition(space_id, discussion_sk);

    AiModeratorMaterial::delete(cli, &pk, Some(EntityType::AiModeratorMaterial(material_id)))
        .await?;

    Ok(())
}
