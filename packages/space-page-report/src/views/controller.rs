use crate::*;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub async fn request_ai_report(
    space_pk: SpacePartition,
) -> Result<controllers::CreateAIReportResponse> {
    let partition = Partition::Space(space_pk.to_string());
    let encoded = utf8_percent_encode(&partition.to_string(), NON_ALPHANUMERIC).to_string();
    controllers::create_ai_report(encoded).await
}

pub async fn get_ai_report(space_pk: SpacePartition) -> Result<controllers::GetAnalyzeResponse> {
    let partition = Partition::Space(space_pk.to_string());
    let encoded = utf8_percent_encode(&partition.to_string(), NON_ALPHANUMERIC).to_string();
    controllers::get_analyze(encoded).await
}

pub async fn save_ai_report(
    space_pk: SpacePartition,
    html_contents: String,
) -> Result<models::SpaceAnalyze> {
    let partition = Partition::Space(space_pk.to_string());
    let encoded = utf8_percent_encode(&partition.to_string(), NON_ALPHANUMERIC).to_string();
    controllers::update_analyze(
        encoded,
        controllers::UpdateAnalyzeHtmlRequest { html_contents },
    )
    .await
}
