use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAIReportRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAIReportResponse {
    pub html_contents: String,
}

// FIXME: implement middleware and authorization
#[cfg(feature = "server")]
#[post("/v3/spaces/{space_pk}/analyze/ai-contents")]
pub async fn create_ai_report(
    space_pk: String,
    req: CreateAIReportRequest,
) -> Result<CreateAIReportResponse, ServerFnError> {
    let config = crate::config::server_config::get();
    let snapshot_key = format!("{}/spaces/{}/snapshots/snapshot.json", config.env, space_pk);

    let s3 = super::utils::s3::build_s3_client().await?;
    let snapshot_json =
        super::utils::s3::load_snapshot_json(&s3, config.bucket_name, &snapshot_key).await?;

    let html_contents =
        super::utils::bedrock::generate_html_contents(&space_pk, &snapshot_json, &req).await?;
    Ok(CreateAIReportResponse { html_contents })
}
