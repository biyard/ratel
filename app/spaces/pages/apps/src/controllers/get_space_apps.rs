use crate::models::SpaceApp;
use crate::*;
use common::SpaceUserRole;
use common::types::Partition;
use common::types::SpacePartition;

#[get("/api/spaces/{space_id}/apps", role: SpaceUserRole)]
pub async fn get_space_apps(space_id: SpacePartition) -> Result<Vec<SpaceApp>> {
    let dynamo = crate::config::get().common.dynamodb();
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
