use crate::*;
use common::types::Partition;
use common::types::SpacePartition;
use common::SpaceUserRole;

#[delete("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn uninstall_space_app(
    space_id: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    let common_config = common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    super::get_apps_access::ensure_space_admin(role)?;

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let app = SpaceApp::delete(dynamo, &pk, Some(sk)).await?;

    Ok(app)
}
