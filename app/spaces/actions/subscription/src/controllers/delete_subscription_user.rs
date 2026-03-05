use crate::models::SpaceSubscriptionUser;
use crate::*;
use common::models::space::SpaceCommon;

#[delete(
    "/api/spaces/{space_id}/subscriptions/users/{user_pk}",
    role: SpaceUserRole
)]
pub async fn delete_subscription_user(
    space_id: SpacePartition,
    user_pk: UserPartition,
) -> Result<()> {
    SpaceSubscription::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_pk: Partition = space_id.clone().into();
    let space = SpaceCommon::get(cli, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    let target_pk: Partition = user_pk.clone().into();
    if target_pk == space.user_pk {
        return Err(Error::BadRequest("Creator cannot be removed".into()));
    }

    let (pk, sk) = SpaceSubscriptionUser::keys(&space_id, &target_pk);
    if SpaceSubscriptionUser::get(cli, &pk, Some(sk.clone()))
        .await?
        .is_none()
    {
        return Ok(());
    }

    SpaceSubscriptionUser::delete(cli, &pk, Some(sk)).await?;
    Ok(())
}
