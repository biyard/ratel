use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::actions::actions::follow::models::*;
use crate::features::spaces::pages::actions::actions::follow::types::SpaceFollowError;
use crate::features::spaces::pages::actions::actions::follow::*;

#[mcp_tool(
    name = "create_follow",
    description = "Create a follow action in a space. Requires creator role."
)]
#[post(
    "/api/spaces/{space_pk}/follows",
    role: SpaceUserRole,
    user: crate::features::auth::User,
    space: SpaceCommon
)]
pub async fn create_follow(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
) -> Result<SpaceFollowAction> {
    SpaceFollowAction::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let pk: Partition = space_pk.clone().into();
    let sk = EntityType::SpaceSubscription;

    if let Some(existing) = SpaceFollowAction::get(cli, &pk, Some(sk.clone())).await? {
        return Ok(existing);
    }

    let follow = SpaceFollowAction::new(space_pk.clone());
    let mut space_action = crate::features::spaces::pages::actions::models::SpaceAction::new(
        space_pk.clone(),
        SpaceActionFollowEntityType::from(follow.sk.clone()).to_string(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
    );
    space_action.title = if space.author_display_name.is_empty() {
        space.author_username
    } else {
        space.author_display_name
    };
    let items = vec![
        follow.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::error!("Failed to create follow: {e}");
        SpaceFollowError::CreateFailed
    })?;

    Ok(follow)
}
