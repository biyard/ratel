use crate::models::SpaceApp;
use crate::*;
use common::types::Partition;
use common::types::SpacePartition;
use common::SpaceUserRole;

#[get("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn get_space_apps(space_id: SpacePartition) -> Result<Vec<SpaceApp>> {
    let common_config = common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    super::get_apps_access::ensure_space_admin(role)?;

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
