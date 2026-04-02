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

    // Server-side premium enforcement
    let membership = crate::features::membership::models::UserMembership::get(
        cli,
        user.pk.clone(),
        Some(EntityType::UserMembership),
    )
    .await?;
    let is_paid = membership
        .as_ref()
        .map_or(false, |m| !m.membership_pk.0.contains("Free"));
    if !is_paid {
        return Err(AiModeratorError::PremiumRequired.into());
    }
    let pk = CompositePartition(space_id, discussion_id.to_string());

    AiModeratorMaterial::delete(cli, &pk, Some(EntityType::AiModeratorMaterial(material_id)))
        .await?;

    Ok(())
}
