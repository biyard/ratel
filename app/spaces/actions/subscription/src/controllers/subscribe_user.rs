use crate::models::SpaceSubscriptionUser;
use crate::*;
use common::models::auth::UserFollow;

#[post(
    "/api/spaces/{space_id}/subscriptions/users/{user_pk}/subscribe",
    role: SpaceUserRole,
    user: ratel_auth::User
)]
pub async fn subscribe_user(space_id: SpacePartition, user_pk: UserPartition) -> Result<()> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let target_pk: Partition = user_pk.clone().into();
    if target_pk == user.pk {
        return Err(Error::BadRequest("Cannot subscribe to yourself".into()));
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
    let target_update = common::models::auth::User::updater(target_pk.clone(), EntityType::User)
        .increase_followers_count(1)
        .transact_write_item();
    let follower_update = common::models::auth::User::updater(user.pk.clone(), EntityType::User)
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
            error!("Failed to subscribe user: {:?}", e);
            Error::Unknown("Failed to subscribe user".into())
        })?;

    Ok(())
}
