use crate::common::models::space::SpaceAdmin;
use crate::features::spaces::pages::apps::apps::general::*;

#[delete("/api/spaces/{space_id}/admins/{user_id}", role: SpaceUserRole)]
pub async fn remove_space_admin(
    space_id: SpacePartition,
    user_id: UserPartition,
) -> Result<()> {
    use crate::common::models::space::SpaceCommon;

    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let user_pk: Partition = user_id.into();

    // Prevent removing the space owner
    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    if space.user_pk == user_pk {
        return Err(Error::BadRequest(
            "Cannot remove the space owner from admins".to_string(),
        ));
    }

    let (pk, sk) = SpaceAdmin::keys(&space_pk, &user_pk);

    SpaceAdmin::get(dynamo, &pk, Some(&sk))
        .await?
        .ok_or(Error::NotFound("Space admin not found".to_string()))?;

    SpaceAdmin::delete(dynamo, &pk, Some(sk)).await?;

    // Also remove the SpaceParticipant record
    use crate::common::models::space::SpaceParticipant;
    let (sp_pk, sp_sk) = SpaceParticipant::keys(space_pk.clone(), user_pk);
    if SpaceParticipant::get(dynamo, &sp_pk, Some(&sp_sk))
        .await?
        .is_some()
    {
        SpaceParticipant::delete(dynamo, &sp_pk, Some(sp_sk)).await?;

        SpaceCommon::updater(&space_pk, &EntityType::SpaceCommon)
            .increase_participants(-1)
            .execute(dynamo)
            .await?;
    }

    Ok(())
}
