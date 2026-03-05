use crate::*;
use common::models::auth::UserFollow;
use ratel_post::models::Team;

#[delete("/api/my-follower/follow", user: ratel_auth::User)]
pub async fn unfollow_user(target_pk: Partition) -> Result<()> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    if target_pk == user.pk {
        return Err(Error::BadRequest("Cannot unfollow yourself".into()));
    }

    let (follower_pk, follower_sk) = UserFollow::follower_keys(&target_pk, &user.pk);
    if UserFollow::get(cli, follower_pk.clone(), Some(follower_sk.clone()))
        .await?
        .is_none()
    {
        return Ok(());
    }

    let (following_pk, following_sk) = UserFollow::following_keys(&user.pk, &target_pk);

    let follower_delete = UserFollow::delete_transact_write_item(&follower_pk, &follower_sk);
    let following_delete = UserFollow::delete_transact_write_item(&following_pk, &following_sk);
    let target_update = match &target_pk {
        Partition::User(_) => {
            common::models::auth::User::updater(target_pk.clone(), EntityType::User)
                .decrease_followers_count(1)
                .transact_write_item()
        }
        Partition::Team(_) => Team::updater(target_pk.clone(), EntityType::Team)
            .decrease_followers(1)
            .transact_write_item(),
        _ => return Err(Error::BadRequest("Invalid target".into())),
    };
    let follower_update = common::models::auth::User::updater(user.pk.clone(), EntityType::User)
        .decrease_followings_count(1)
        .transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![
            follower_delete,
            following_delete,
            target_update,
            follower_update,
        ]))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to unfollow user: {:?}", e);
            Error::Unknown("Failed to unfollow user".into())
        })?;

    Ok(())
}
