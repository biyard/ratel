use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::actions::actions::subscription::models::SpaceSubscriptionUser;
use crate::features::spaces::pages::actions::actions::subscription::*;

#[delete("/api/spaces/{space_id}/subscriptions/users", role: SpaceUserRole)]
pub async fn delete_subscription_user(space_id: SpacePartition, user_pk: Partition) -> Result<()> {
    SpaceSubscriptionAction::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_pk: Partition = space_id.clone().into();
    let space = SpaceCommon::get(cli, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    if user_pk == space.user_pk {
        return Err(Error::BadRequest("Creator cannot be removed".into()));
    }

    let (pk, sk) = SpaceSubscriptionUser::keys(&space_id, &user_pk);
    if SpaceSubscriptionUser::get(cli, &pk, Some(sk.clone()))
        .await?
        .is_none()
    {
        return Ok(());
    }

    SpaceSubscriptionUser::delete(cli, &pk, Some(sk)).await?;
    Ok(())
}
