use crate::features::spaces::analyzes::SpaceAnalyze;
// use crate::models::SpaceCommon;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::reports::build_space_report_pdf;
use crate::utils::reports::upload_report_pdf_to_s3;
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
    let pdf_bytes = build_space_report_pdf(
        &analyze.lda_topics,
        analyze.lda_html_contents.unwrap_or_default(),
        analyze.network,
        analyze.network_html_contents.unwrap_or_default(),
        &analyze.tf_idf,
        analyze.tf_idf_html_contents.unwrap_or_default(),
    )
    .await?;
    let (_key, uri) = upload_report_pdf_to_s3(pdf_bytes).await?;

    let _ = SpaceAnalyze::updater(space_pk, EntityType::SpaceAnalyze)
        .with_metadata_url(uri.clone())
        .execute(&dynamo.client)
        .await?;
    Ok(Json(DownloadAnalyzeResponse { metadata_url: uri }))
}
