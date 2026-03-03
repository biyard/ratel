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
    SpaceApp::can_edit(role)?;

    let common_config = common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk_partition: Partition = space_id.clone().into();

    let app = SpaceApp::new(space_pk_partition, app_type);
    app.create(dynamo).await?;

    Ok(app)
}
