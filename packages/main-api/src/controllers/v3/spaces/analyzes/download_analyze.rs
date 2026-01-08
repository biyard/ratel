use crate::features::spaces::analyzes::SpaceAnalyze;
// use crate::models::SpaceCommon;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::reports::build_space_report_pdf;
use crate::utils::reports::upload_report_pdf_to_s3;
use crate::*;

pub async fn download_analyze_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(SpacePathParam { space_pk }): SpacePath,
    // Extension(space): Extension<SpaceCommon>,
) -> Result<Json<String>> {
    if !matches!(space_pk.clone(), Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let analyze =
        SpaceAnalyze::get(&dynamo.client, &space_pk, Some(EntityType::SpaceAnalyze)).await?;

    if analyze.is_none() {
        return Err(Error::AnalyzeNotFound);
    }

    let analyze = analyze.unwrap();
    let pdf_bytes =
        build_space_report_pdf(&analyze.lda_topics, &analyze.network.nodes, &analyze.tf_idf)?;
    let (_key, uri) = upload_report_pdf_to_s3(pdf_bytes).await?;

    Ok(Json(uri))
}
