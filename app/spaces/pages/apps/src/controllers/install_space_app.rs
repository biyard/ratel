#[cfg(feature = "server")]
use super::{ensure_space_exists, parse_app_name};
#[cfg(feature = "server")]
use crate::models::SpaceInstalledApp;
use crate::*;
#[cfg(feature = "server")]
use common::types::Partition;
use common::types::SpacePartition;

#[post("/api/spaces/{space_pk}/apps/{app_name}/install")]
pub async fn install_space_app(
    space_pk: SpacePartition,
    app_name: String,
) -> Result<SpaceAppMutationResponse> {
    let dynamo = crate::config::get().common.dynamodb();
    let space_pk_partition: Partition = space_pk.into();
    #[cfg(feature = "server")]
    ensure_space_exists(dynamo, &space_pk_partition).await?;

    let app_name = parse_app_name(&app_name)?;

    let (pk, sk) = SpaceInstalledApp::keys(&space_pk_partition, app_name);
    let app = SpaceInstalledApp::get(dynamo, &pk, Some(&sk)).await?;

    if app.is_none() {
        SpaceInstalledApp::new(space_pk_partition, app_name)
            .create(dynamo)
            .await?;
    }

    Ok(SpaceAppMutationResponse {
        name: app_name,
        installed: true,
    })
}
