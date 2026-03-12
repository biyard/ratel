use crate::common::models::space::SpaceCommon;
use crate::features::posts::models::Team;
use crate::features::spaces::pages::actions::actions::follow::models::*;
use crate::features::spaces::pages::actions::actions::follow::*;

#[post(
    "/api/spaces/{space_pk}/follows",
    role: SpaceUserRole,
    user: crate::features::auth::User
)]
pub async fn create_follow(space_pk: SpacePartition) -> Result<SpaceFollowAction> {
    use crate::features::auth::{User, UserQueryOption};

    SpaceFollowAction::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let pk: Partition = space_pk.clone().into();
    let sk = EntityType::SpaceSubscription;

    if let Some(existing) = SpaceFollowAction::get(cli, &pk, Some(sk.clone())).await? {
        return Ok(existing);
    }

    let follow = SpaceFollowAction::new(space_pk.clone());
    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::new(
        space_pk.clone(),
        SpaceActionFollowEntityType::from(follow.sk.clone()).to_string(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
    );
    let items = vec![
        follow.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::features::spaces::pages::actions::actions::follow::Error::Unknown(format!(
            "Failed to create follow: {e}"
        ))
    })?;

    Ok(follow)
}
