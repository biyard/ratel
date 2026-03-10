use crate::common::models::space::SpaceCommon;
use crate::features::posts::models::Team;
use crate::features::spaces::pages::actions::actions::subscription::models::*;
use crate::features::spaces::pages::actions::actions::subscription::*;

#[post(
    "/api/spaces/{space_pk}/subscriptions",
    role: SpaceUserRole,
    user: crate::features::auth::User
)]
pub async fn create_subscription(space_pk: SpacePartition) -> Result<SpaceSubscriptionAction> {
    use crate::features::auth::{User, UserQueryOption};

    SpaceSubscriptionAction::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let pk: Partition = space_pk.clone().into();
    let sk = EntityType::SpaceSubscription;

    if let Some(existing) = SpaceSubscriptionAction::get(cli, &pk, Some(sk.clone())).await? {
        return Ok(existing);
    }

    let subscription = SpaceSubscriptionAction::new(space_pk.clone());
    subscription.create(cli).await?;

    Ok(subscription)
}
