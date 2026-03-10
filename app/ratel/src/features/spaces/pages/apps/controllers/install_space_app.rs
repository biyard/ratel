use crate::features::spaces::pages::apps::models::SpaceApp;
use crate::features::spaces::pages::apps::*;
use crate::common::SpaceUserRole;
use crate::common::types::Partition;
use crate::common::types::SpacePartition;

#[post("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn install_space_app(
    space_id: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    SpaceApp::can_edit(role)?;

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    let app = SpaceApp::new(space_pk_partition, app_type);

    let items = vec![app.create_transact_write_item()];
    crate::transact_write_items!(dynamo, items)
        .map_err(|e| crate::features::spaces::pages::apps::Error::Unknown(format!("Failed to install app: {e}")))?;

    Ok(app)
}
