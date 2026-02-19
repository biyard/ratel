use crate::*;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub async fn request_ai_report(
    space_pk: SpacePartition,
) -> std::result::Result<controllers::CreateAIReportResponse, ServerFnError> {
    let partition = Partition::Space(space_pk.to_string());
    let encoded = utf8_percent_encode(&partition.to_string(), NON_ALPHANUMERIC).to_string();
    controllers::create_ai_report(encoded).await
}
