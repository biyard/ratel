use crate::features::spaces::pages::apps::models::SpaceApp;
use crate::features::spaces::pages::apps::*;
use crate::common::SpaceUserRole;
use crate::common::types::Partition;
use crate::common::types::SpacePartition;

#[mcp_tool(name = "install_space_app", description = "Install an app in a space. Requires creator role. Types: General, File, Analyzes, Panels.")]
#[post("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn install_space_app(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "App type: 'General', 'File', 'Analyzes', or 'Panels'")]
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    SpaceApp::can_edit(role)?;

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    let app = SpaceApp::new(space_pk_partition, app_type);

    let items = vec![app.create_transact_write_item()];
    crate::transact_write_items!(dynamo, items)
        .map_err(|e| {
            crate::error!("Failed to install app: {e}");
            SpaceAppError::InstallFailed
        })?;

    Ok(app)
}
