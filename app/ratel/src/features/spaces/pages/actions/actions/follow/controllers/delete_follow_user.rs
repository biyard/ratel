use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::actions::actions::follow::models::SpaceFollowUser;
use crate::features::spaces::pages::actions::actions::follow::*;

#[delete("/api/spaces/{space_id}/follows/users", role: SpaceUserRole)]
pub async fn delete_follow_user(space_id: SpacePartition, user_pk: Partition) -> Result<()> {
    SpaceFollowAction::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_pk: Partition = space_id.clone().into();
    let space = SpaceCommon::get(cli, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    if user_pk == space.user_pk {
        return Err(Error::BadRequest("Creator cannot be removed".into()));
    }

    let (pk, sk) = SpaceFollowUser::keys(&space_id, &user_pk);
    if SpaceFollowUser::get(cli, &pk, Some(sk.clone()))
        .await?
        .is_none()
    {
        return Ok(());
    }

    SpaceFollowUser::delete(cli, &pk, Some(sk)).await?;
    Ok(())
}
