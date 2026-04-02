use crate::features::ai_moderator::*;

#[delete("/api/spaces/{space_id}/discussions/{discussion_id}/ai-moderator/materials/{material_id}", role: SpaceUserRole, user: crate::features::auth::User)]
pub async fn delete_material(
    space_id: SpacePartition,
    discussion_id: SpaceDiscussionEntityType,
    material_id: String,
) -> Result<()> {
    role.is_creator()?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    super::require_premium(cli, &user).await?;

    let pk = CompositePartition(space_id, discussion_id.to_string());

    AiModeratorMaterial::delete(cli, &pk, Some(EntityType::AiModeratorMaterial(material_id)))
        .await?;

    Ok(())
}
