use crate::models::*;
use crate::*;
use common::models::space::SpaceCommon;

#[post(
    "/api/spaces/{space_pk}/subscriptions",
    role: SpaceUserRole,
    user: ratel_auth::User
)]
pub async fn create_subscription(space_pk: SpacePartition) -> Result<SpaceSubscription> {
    use ratel_auth::User;

    SpaceSubscription::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let pk: Partition = space_pk.clone().into();
    let sk = EntityType::SpaceSubscription;

    if let Some(existing) = SpaceSubscription::get(cli, &pk, Some(sk.clone())).await? {
        return Ok(existing);
    }

    let subscription = SpaceSubscription::new(space_pk.clone());
    subscription.create(cli).await?;

    let space = SpaceCommon::get(cli, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    let user = User::get(cli, &space.user_pk, Some(EntityType::User))
        .await?
        .unwrap_or_default();
    let creator = SpaceSubscriptionUser::new(space_pk, &user);
    creator.upsert(cli).await?;

    Ok(subscription)
}
