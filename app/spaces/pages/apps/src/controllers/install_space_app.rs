use crate::models::SpaceApp;
use crate::*;
use common::types::Partition;
use common::types::SpacePartition;

#[post("/api/spaces/{space_id}/apps")]
pub async fn install_space_app(
    space_id: SpacePartition,
    app_type: SpaceAppType,
) -> Result<SpaceApp> {
    use super::ensure_space_exists;

    let dynamo = crate::config::get().common.dynamodb();
    let space_pk_partition: Partition = space_id.into();
    #[cfg(feature = "server")]
    ensure_space_exists(dynamo, &space_pk_partition).await?;

    let (pk, sk) = SpaceApp::keys(&space_pk_partition, app_type);
    let app = SpaceApp::get(dynamo, &pk, Some(&sk)).await?;

    if app.is_none() {
        SpaceApp::new(space_pk_partition, app_type)
            .create(dynamo)
            .await?;
    } else {
        return Err(Error::Duplicate("App already exists".to_string()));
    }

    Ok(app.unwrap())
}
