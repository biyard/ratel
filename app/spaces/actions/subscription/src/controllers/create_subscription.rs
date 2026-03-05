use crate::models::*;
use crate::*;

#[post("/api/spaces/{space_pk}/subscriptions", role: SpaceUserRole)]
pub async fn create_subscription(space_pk: SpacePartition) -> Result<SpaceSubscription> {
    SpaceSubscription::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let pk: Partition = space_pk.clone().into();
    let sk = EntityType::SpaceSubscription;

    if let Some(existing) = SpaceSubscription::get(cli, &pk, Some(sk.clone())).await? {
        return Ok(existing);
    }

    let subscription = SpaceSubscription::new(space_pk);
    subscription.create(cli).await?;

    Ok(subscription)
}
