use crate::features::spaces::analyzes::SpaceAnalyze;
// use crate::models::SpaceCommon;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::reports::build_report_html_document;
use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct DownloadAnalyzeResponse {
    pub presigned_url: String,
    pub metadata_url: String,
    pub html_document: String,
}

pub async fn download_analyze_handler(
    State(AppState { dynamo, s3, .. }): State<AppState>,
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
    let html_contents = analyze.html_contents.unwrap_or_default();
    let html_document = build_report_html_document(&html_contents);
    let upload = s3.presign_report_upload().await?;

    Ok(Json(DownloadAnalyzeResponse {
        presigned_url: upload.presigned_url,
        metadata_url: upload.metadata_url,
        html_document,
    }))
}
