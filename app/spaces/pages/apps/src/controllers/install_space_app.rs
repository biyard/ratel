use crate::models::SpaceApp;
use crate::*;
use common::types::Partition;
use common::types::SpacePartition;
use ratel_auth::models::user::OptionalUser;

#[post("/api/spaces/{space_id}/apps", user: OptionalUser)]
pub async fn install_space_app(
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
    let existing = SpaceApp::get(dynamo, &pk, Some(&sk)).await?;

    if existing.is_some() {
        return Err(Error::Duplicate("App already exists".to_string()));
    }

    let app = SpaceApp::new(space_pk_partition, app_type);
    app.create(dynamo).await?;

    Ok(app)
}
