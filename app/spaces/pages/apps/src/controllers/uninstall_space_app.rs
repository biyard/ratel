use crate::*;

use common::types::Partition;
use common::types::SpacePartition;

#[delete("/api/spaces/{space_id}/apps", _space: SpaceCommon)]
pub async fn uninstall_space_app(
    space_id: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    let dynamo = crate::config::get().common.dynamodb();
    let space_pk_partition: Partition = space_id.into();

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let app = SpaceApp::delete(dynamo, &pk, Some(sk)).await?;

    Ok(app)
}
