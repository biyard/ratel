use crate::models::*;
use crate::*;
use common::models::space::SpaceCommon;
use ratel_post::models::Team;

#[post(
    "/api/spaces/{space_pk}/subscriptions",
    role: SpaceUserRole,
    user: ratel_auth::User
)]
pub async fn create_subscription(space_pk: SpacePartition) -> Result<SpaceSubscription> {
    use ratel_auth::{User, UserQueryOption};

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

    Ok(subscription)
}
