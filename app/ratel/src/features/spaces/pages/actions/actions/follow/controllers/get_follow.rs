use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[get("/api/spaces/{space_pk}/follows/{follow_id}", role: SpaceUserRole)]
pub async fn get_follow(
    space_pk: SpacePartition,
    follow_id: SpaceActionFollowEntityType,
) -> Result<SpaceAction> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_action = SpaceAction::get(
        cli,
        &CompositePartition::<SpacePartition, String>(space_pk.into(), follow_id.to_string()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::NotFound("Follow action not found".into()))?;

    Ok(space_action)
}
