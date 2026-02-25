use crate::*;
#[cfg(feature = "server")]
use common::types::Partition;
use ratel_auth::models::user::OptionalUser;

#[get("/api/spaces/{space_id}/apps/access", user: OptionalUser)]
pub async fn get_apps_access(space_id: SpacePartition) -> Result<bool> {
    #[cfg(feature = "server")]
    {
        let user: Option<ratel_auth::User> = user.into();
        let is_admin = resolve_space_admin(space_id, user).await.unwrap_or(false);
        return Ok(is_admin);
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = (space_id, user);
        Ok(false)
    }
}

#[cfg(feature = "server")]
pub(super) async fn ensure_space_admin(
    space_id: SpacePartition,
    user: Option<ratel_auth::User>,
) -> Result<()> {
    let user = user.ok_or(Error::NoSessionFound)?;
    let is_admin = resolve_space_admin(space_id, Some(user)).await?;

    if !is_admin {
        return Err(Error::NoPermission);
    }

    Ok(())
}

#[cfg(feature = "server")]
async fn resolve_space_admin(
    space_id: SpacePartition,
    user: Option<ratel_auth::User>,
) -> Result<bool> {
    use common::models::space::SpaceCommon;
    use space_common::ratel_post::{models::Team, types::TeamGroupPermission};

    let Some(user) = user else {
        return Ok(false);
    };

    let dynamo = crate::config::get().common.dynamodb();
    let space_pk: Partition = space_id.into();

    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    if user.pk == space.user_pk {
        return Ok(true);
    }

    if matches!(space.user_pk, Partition::Team(_)) {
        return Team::has_permission(
            dynamo,
            &space.user_pk,
            &user.pk,
            TeamGroupPermission::TeamAdmin,
        )
        .await;
    }

    Ok(false)
}
