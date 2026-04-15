use crate::common::models::auth::UserFollow;
use crate::common::models::space::SpaceCommon;
use crate::features::posts::models::Team;
use crate::features::spaces::pages::actions::actions::follow::models::SpaceFollowUser;
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::actions::follow::types::SpaceFollowError;

#[mcp_tool(name = "follow_user", description = "Follow a user as part of a space follow action. Requires participant role.")]
#[post(
    "/api/spaces/{space_id}/follows/{follow_id}/user",
    role: SpaceUserRole,
    user: crate::features::auth::User,
    member: crate::common::models::space::SpaceUser,
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
        return Err(SpaceFollowError::CannotFollowSelf.into());
    }

    let is_allowed_target = if target_pk == space.user_pk {
        true
    } else {
        let (pk, sk) = SpaceFollowUser::keys(&space_id, &target_pk);
        SpaceFollowUser::get(cli, &pk, Some(sk)).await?.is_some()
    };

    if !is_allowed_target {
        return Err(SpaceFollowError::InvalidFollowTarget.into());
    }

    let (follower_pk, follower_sk) = UserFollow::follower_keys(&target_pk, &user.pk);
    if UserFollow::get(cli, follower_pk.clone(), Some(follower_sk.clone()))
        .await?
        .is_some()
    {
        return Ok(());
    }

    let (follower_record, following_record) =
        UserFollow::new_follow_records_with_space(
            user.pk.clone(),
            target_pk.clone(),
            Some(space_id.to_string()),
            Some(follow_id.to_string()),
            Some(member.display_name.clone()),
            Some(member.profile_url.clone()),
        );

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
        _ => return Err(SpaceFollowError::InvalidTarget.into()),
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
            crate::error!("Failed to follow user: {:?}", e);
            SpaceFollowError::FollowFailed
        })?;

    // Reward payout + XP recording run on EventBridge via FOLLOWER# INSERT →
    // handle_follow_xp. See features/activity/services/handle_xp_event.rs.

    Ok(())
}
