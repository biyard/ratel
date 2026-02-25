use crate::models::SpaceApp;
use crate::*;
use common::types::Partition;
use common::types::SpacePartition;

#[get("/api/spaces/{space_id}/apps")]
pub async fn get_space_apps(space_id: SpacePartition) -> Result<Vec<SpaceApp>> {
    use super::ensure_space_exists;
    let dynamo = crate::config::get().common.dynamodb();
    let space_pk: Partition = space_id.into();
    ensure_space_exists(dynamo, &space_pk).await?;

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
