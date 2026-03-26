use crate::common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::space_reward::SpaceReward;
use crate::features::spaces::pages::actions::actions::follow::models::*;
use crate::features::spaces::pages::actions::actions::follow::*;

#[post(
    "/api/spaces/{space_pk}/follows",
    role: SpaceUserRole,
    user: crate::features::auth::User,
    space: SpaceCommon
)]
pub async fn create_follow(space_pk: SpacePartition) -> Result<SpaceFollowAction> {
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
        crate::features::spaces::pages::actions::actions::follow::Error::Unknown(format!(
            "Failed to create follow: {e}"
        ))
    })?;

    let action_id = SpaceActionFollowEntityType::from(follow.sk.clone()).to_string();
    match SpaceReward::get_by_action(
        cli,
        space_pk.clone(),
        action_id.clone(),
        RewardUserBehavior::Follow,
    )
    .await
    {
        Ok(space_reward) => {
            if let Err(e) = SpaceReward::award(
                cli,
                &space_reward,
                user.pk.clone(),
                Some(space.user_pk.clone()),
            )
            .await
            {
                tracing::error!(
                    space_pk = %space_pk,
                    action_id = %action_id,
                    error = %e,
                    "Failed to award follow reward"
                );
            }
        }
        Err(e) => {
            tracing::warn!(
                space_pk = %space_pk,
                action_id = %action_id,
                error = %e,
                "SpaceReward not found for follow action"
            );
        }
    }

    Ok(follow)
}
