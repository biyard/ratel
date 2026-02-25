#[cfg(feature = "server")]
use common::{Error, SpacePartition};
#[cfg(feature = "server")]
use ratel_auth::User;

mod delete_space;
mod get_space_administrator;
mod invite_space_participants;

pub use delete_space::*;
pub use get_space_administrator::*;
pub use invite_space_participants::*;

#[cfg(feature = "server")]
pub(super) async fn get_space_and_ensure_admin(
    space_id: &SpacePartition,
    user: &User,
) -> common::Result<common::models::space::SpaceCommon> {
    use common::models::space::SpaceCommon;
    use common::types::{EntityType, Partition};
    use ratel_post::{models::Team, types::TeamGroupPermission};

    let dynamo = crate::config::get().common.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    if space.user_pk == user.pk {
        return Ok(space);
    }

    if matches!(&space.user_pk, Partition::Team(_))
        && Team::has_permission(
            dynamo,
            &space.user_pk,
            &user.pk,
            TeamGroupPermission::TeamAdmin,
        )
        .await?
    {
        return Ok(space);
    }

    Err(Error::NoPermission)
}
