use crate::*;
use common::types::Partition;
use common::types::SpacePartition;
use ratel_auth::models::user::OptionalUser;

#[delete("/api/spaces/{space_id}/apps", user: OptionalUser)]
pub async fn uninstall_space_app(
    space_id: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    let dynamo = crate::config::get().common.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    #[cfg(feature = "server")]
    {
        let user: Option<ratel_auth::User> = user.into();
        super::get_apps_access::ensure_space_admin(space_id, user).await?;
    }

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let app = SpaceApp::delete(dynamo, &pk, Some(sk)).await?;

    Ok(app)
}
