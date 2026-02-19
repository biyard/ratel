use crate::*;

pub async fn request_ai_report(
    space_pk: SpacePartition,
) -> Result<controllers::CreateAIReportResponse> {
    controllers::create_ai_report(space_pk).await
}

pub async fn get_ai_report(space_pk: SpacePartition) -> Result<controllers::GetAnalyzeResponse> {
    controllers::get_analyze(space_pk).await
}

pub async fn save_ai_report(
    space_pk: SpacePartition,
    html_contents: String,
) -> Result<models::SpaceAnalyze> {
    controllers::update_analyze(
        space_pk,
        controllers::UpdateAnalyzeHtmlRequest { html_contents },
    )
    .await
}
