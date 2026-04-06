use crate::common::models::auth::UserFollow;
use crate::common::models::space::SpaceCommon;
use crate::features::posts::models::Team;
use crate::features::spaces::pages::actions::actions::follow::models::SpaceFollowUser;
use crate::features::spaces::pages::actions::actions::follow::*;
#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::space_reward::SpaceReward;

#[mcp_tool(name = "follow_user", description = "Follow a user as part of a space follow action. Requires participant role.")]
#[post(
    "/api/spaces/{space_id}/follows/{follow_id}/user",
    role: SpaceUserRole,
    user: crate::features::auth::User,
    author: crate::common::models::space::SpaceAuthor,
    space: SpaceCommon
)]
pub async fn follow_user(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Follow action sort key (e.g. 'SpaceActionFollow#<uuid>')")]
    follow_id: SpaceActionFollowEntityType,
    #[mcp(description = "Target user partition key to follow (e.g. 'USER#<uuid>')")]
    target_pk: Partition,
) -> Result<()> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    if target_pk == user.pk {
        return Err(Error::BadRequest("Cannot follow yourself".into()));
    }

    let is_allowed_target = if target_pk == space.user_pk {
        true
    } else {
        let (pk, sk) = SpaceFollowUser::keys(&space_id, &target_pk);
        SpaceFollowUser::get(cli, &pk, Some(sk)).await?.is_some()
    };

    if !is_allowed_target {
        return Err(Error::BadRequest("Invalid follow target".into()));
    }

    let (follower_pk, follower_sk) = UserFollow::follower_keys(&target_pk, &user.pk);
    if UserFollow::get(cli, follower_pk.clone(), Some(follower_sk.clone()))
        .await?
        .is_some()
    {
        return Ok(());
    }

    let (follower_record, following_record) =
        UserFollow::new_follow_records(user.pk.clone(), target_pk.clone());

    let follow_tx = follower_record.create_transact_write_item();
    let following_tx = following_record.create_transact_write_item();
    let target_update = match &target_pk {
        Partition::User(_) => {
            crate::common::models::auth::User::updater(target_pk.clone(), EntityType::User)
                .increase_followers_count(1)
                .transact_write_item()
        }
        Partition::Team(_) => Team::updater(target_pk.clone(), EntityType::Team)
            .increase_followers(1)
            .transact_write_item(),
        _ => return Err(Error::BadRequest("Invalid target".into())),
    };
    let follower_update =
        crate::common::models::auth::User::updater(user.pk.clone(), EntityType::User)
            .increase_followings_count(1)
            .transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![
            follow_tx,
            following_tx,
            target_update,
            follower_update,
        ]))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to follow user: {:?}", e);
            Error::Unknown("Failed to follow user".into())
        })?;

    match SpaceReward::get_by_action(
        cli,
        space_id.clone(),
        follow_id.to_string(),
        RewardUserBehavior::Follow,
    )
    .await
    {
        Ok(space_reward) => {
            if let Err(e) =
                SpaceReward::award(cli, &space_reward, user.pk.clone(), Some(space.user_pk.clone()))
                    .await
            {
                tracing::error!(
                    space_pk = %space_id,
                    action_id = %follow_id,
                    target_pk = %target_pk,
                    error = %e,
                    "Failed to award follow reward"
                );
            }
        }
        Err(e) => {
            tracing::warn!(
                space_pk = %space_id,
                action_id = %follow_id,
                target_pk = %target_pk,
                error = %e,
                "SpaceReward not found for follow action"
            );
        }
    }

    {
        let follow_space_action = crate::features::spaces::pages::actions::models::SpaceAction::get(
            cli,
            &CompositePartition(space_id.clone(), follow_id.to_string()),
            Some(EntityType::SpaceAction),
        ).await.ok().flatten();
        if let Some(ref sa) = follow_space_action {
            if let Err(e) = crate::features::activity::controllers::record_activity(
                cli,
                space_id.clone(),
                crate::features::activity::types::AuthorPartition::from(user.pk.clone()),
                follow_id.to_string(),
                crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
                sa.activity_score,
                sa.additional_score,
                crate::features::activity::types::SpaceActivityData::Follow {
                    follow_id: follow_id.to_string(),
                },
                author.display_name.clone(),
                author.profile_url.clone(),
            ).await {
                tracing::error!(error = %e, "Failed to record follow activity");
            }
        }
    }

    Ok(())
}
