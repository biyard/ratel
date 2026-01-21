use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::reports::presign_report_download;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct DownloadAnalyzeUrlResponse {
    pub download_url: String,
}

pub async fn download_analyze_url_handler(
    State(AppState { dynamo, s3, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<DownloadAnalyzeUrlResponse>> {
    if !matches!(space_pk.clone(), Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let analyze =
        SpaceAnalyze::get(&dynamo.client, &space_pk, Some(EntityType::SpaceAnalyze)).await?;
    let analyze = analyze.ok_or(Error::AnalyzeNotFound)?;

    let metadata_url = analyze
        .metadata_url
        .unwrap_or_default()
        .trim()
        .to_string();
    if metadata_url.is_empty() {
        return Err(Error::AnalyzeNotFound);
    }

    let key = metadata_url
        .splitn(2, "://")
        .nth(1)
        .and_then(|rest| rest.splitn(2, '/').nth(1))
        .ok_or_else(|| Error::InternalServerError("Invalid metadata_url".to_string()))?;

    let download_url = presign_report_download(&s3, key, "analysis-report.pdf").await?;

    Ok(Json(DownloadAnalyzeUrlResponse { download_url }))
}
