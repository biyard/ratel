use crate::features::spaces::pages::apps::models::SpaceApp;
use crate::features::spaces::pages::apps::*;
use crate::common::SpaceUserRole;
use crate::common::types::Partition;
use crate::common::types::SpacePartition;

#[mcp_tool(name = "uninstall_space_app", description = "Uninstall an app from a space. Requires creator role.")]
#[delete("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn uninstall_space_app(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "App type: 'General', 'File', 'Analyzes', or 'Panels'")]
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    SpaceApp::can_delete(role)?;

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let app = SpaceApp::new(space_pk_partition, app_type);

    let items = vec![SpaceApp::delete_transact_write_item(&pk, sk)];
    crate::transact_write_items!(dynamo, items)
        .map_err(|e| crate::features::spaces::pages::apps::Error::Unknown(format!("Failed to uninstall app: {e}")))?;

    Ok(app)
}
