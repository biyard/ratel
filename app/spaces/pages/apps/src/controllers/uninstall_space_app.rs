use crate::*;

use common::types::Partition;
use common::types::SpacePartition;

#[delete("/api/spaces/{space_pk}/apps")]
pub async fn uninstall_space_app(
    space_pk: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    use super::ensure_space_exists;

    let dynamo = crate::config::get().common.dynamodb();
    let space_pk_partition: Partition = space_pk.into();

    ensure_space_exists(dynamo, &space_pk_partition).await?;

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let app = SpaceApp::delete(dynamo, &pk, Some(sk)).await?;

    Ok(app)
}
