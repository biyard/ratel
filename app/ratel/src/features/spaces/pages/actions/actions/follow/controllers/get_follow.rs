use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[mcp_tool(name = "get_follow", description = "Get follow action details including the target user to follow.")]
#[get("/api/spaces/{space_pk}/follows/{follow_id}", role: SpaceUserRole)]
pub async fn get_follow(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
    #[mcp(description = "Follow action sort key (e.g. 'SpaceActionFollow#<uuid>')")]
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
