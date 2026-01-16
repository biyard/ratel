use crate::features::spaces::analyzes::SpaceAnalyze;
// use crate::models::SpaceCommon;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::aws::PollScheduler;
use crate::utils::aws::get_aws_config;
use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct DownloadAnalyzeResponse {
    pub metadata_url: String,
}

pub async fn download_analyze_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    // Extension(space): Extension<SpaceCommon>,
) -> Result<Json<DownloadAnalyzeResponse>> {
    if !matches!(space_pk.clone(), Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let analyze =
        SpaceAnalyze::get(&dynamo.client, &space_pk, Some(EntityType::SpaceAnalyze)).await?;

    if analyze.is_none() {
        return Err(Error::AnalyzeNotFound);
    }

    let analyze = analyze.unwrap();

    if analyze.metadata_url.is_some() {
        let metadata_url = analyze.metadata_url.unwrap();

        if metadata_url != "".to_string() && metadata_url != "pending".to_string() {
            return Ok(Json(DownloadAnalyzeResponse { metadata_url }));
        }
    }

    let pending_url = "pending".to_string();

    let _ = SpaceAnalyze::updater(&space_pk, EntityType::SpaceAnalyze)
        .with_metadata_url(pending_url.clone())
        .execute(&dynamo.client)
        .await?;

    let sdk_config = get_aws_config();
    let scheduler = PollScheduler::new(&sdk_config);
    scheduler
        .schedule_download_analyze(space_pk.clone())
        .await?;

    Ok(Json(DownloadAnalyzeResponse {
        metadata_url: pending_url,
    }))
}
