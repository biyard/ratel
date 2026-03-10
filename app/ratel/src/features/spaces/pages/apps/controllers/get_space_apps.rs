use crate::features::spaces::pages::apps::models::SpaceApp;
use crate::features::spaces::pages::apps::*;
use crate::common::SpaceUserRole;
use crate::common::types::Partition;
use crate::common::types::SpacePartition;

#[get("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn get_space_apps(space_id: SpacePartition) -> Result<Vec<SpaceApp>> {
    SpaceApp::can_view(role)?;

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    let (apps, _) = SpaceApp::query(
        dynamo,
        &space_pk,
        SpaceApp::opt_all()
            .sk(SpaceApp::sk_prefix())
            .scan_index_forward(true),
    )
    .await?;

    Ok(apps)
}
