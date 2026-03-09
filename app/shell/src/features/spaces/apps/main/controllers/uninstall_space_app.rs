use crate::features::spaces::apps::main::models::SpaceApp;
use crate::features::spaces::apps::main::*;
use common::SpaceUserRole;
use common::types::Partition;
use common::types::SpacePartition;

#[delete("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn uninstall_space_app(
    space_id: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    SpaceApp::can_delete(role)?;

    let common_config = common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let app = SpaceApp::new(space_pk_partition, app_type);

    let items = vec![SpaceApp::delete_transact_write_item(&pk, sk)];
    transact_write_items!(dynamo, items)
        .map_err(|e| crate::features::spaces::apps::main::Error::Unknown(format!("Failed to uninstall app: {e}")))?;

    Ok(app)
}
