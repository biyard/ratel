use crate::models::SpaceApp;
use crate::*;
use common::SpaceUserRole;
use common::types::Partition;
use common::types::SpacePartition;

#[post("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn install_space_app(
    space_id: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    let dynamo = crate::config::get().common.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    super::get_apps_access::ensure_space_admin(role)?;

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let existing = SpaceApp::get(dynamo, &pk, Some(&sk)).await?;

    if existing.is_some() {
        return Err(Error::Duplicate("App already exists".to_string()));
    }

    let app = SpaceApp::new(space_pk_partition, app_type);
    app.create(dynamo).await?;

    Ok(app)
}
