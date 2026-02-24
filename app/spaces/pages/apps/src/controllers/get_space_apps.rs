#[cfg(feature = "server")]
use super::ensure_space_exists;
#[cfg(feature = "server")]
use crate::models::SpaceInstalledApp;
use crate::*;
#[cfg(feature = "server")]
use common::types::Partition;
use common::types::SpacePartition;

#[get("/api/spaces/{space_pk}/apps")]
pub async fn get_space_apps(space_pk: SpacePartition) -> Result<GetSpaceAppsResponse> {
    let dynamo = crate::config::get().common.dynamodb();
    let space_pk_partition: Partition = space_pk.into();
    #[cfg(feature = "server")]
    ensure_space_exists(dynamo, &space_pk_partition).await?;

    let (apps, _) = SpaceInstalledApp::query(
        dynamo,
        &space_pk_partition,
        SpaceInstalledApp::opt_all()
            .sk(SpaceInstalledApp::sk_prefix())
            .scan_index_forward(true),
    )
    .await?;

    let apps = apps
        .into_iter()
        .map(|app| InstalledAppResponse { name: app.name })
        .collect::<Vec<_>>();

    Ok(GetSpaceAppsResponse { apps })
}
