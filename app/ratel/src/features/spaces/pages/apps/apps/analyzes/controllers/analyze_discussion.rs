//! Detail-page on-demand discussion analysis endpoint.
//!
//! POST creates a fresh `SpaceAnalyzeDiscussionResult` row with the
//! user-supplied params and `status=InProgress`. The DDB stream Pipe
//! attached to the result entity's INSERT events fires the
//! `AnalyzeDiscussionInProgress` Lambda, which loads the matched users'
//! comments on the target discussion, runs the text-analysis pipeline
//! (lindera + Gibbs LDA + TF-IDF + text-network), and overwrites the
//! same row's result fields + flips status to `Finish`.
//!
//! The response carries the row id (UUIDv7-based request_id) so the
//! frontend can immediately switch to a polling/refetch loop on the
//! detail page panel.

use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeDiscussionRequest {
    pub params: DiscussionAnalysisParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeDiscussionResponse {
    pub request_id: String,
    pub report_id: String,
    pub discussion_id: String,
}

#[mcp_tool(
    name = "analyze_discussion",
    description = "Enqueue a fresh discussion text-analysis run (LDA + TF-IDF + text-network) for the matched users on one discussion under a finished analyze report. Requires creator role."
)]
#[post(
    "/api/spaces/{space_id}/apps/analyzes/reports/{report_id}/discussions/{discussion_id}/analyze",
    role: SpaceUserRole
)]
pub async fn analyze_discussion(
    #[mcp(description = "Space partition key")] space_id: SpacePartition,
    #[mcp(description = "Analyze report id")] report_id: SpaceAnalyzeReportEntityType,
    #[mcp(description = "Discussion (post) partition key")] discussion_id: FeedPartition,
    #[mcp(
        description = "Analysis params payload as JSON, e.g. {\"params\": {\"top_k_keywords\": 30, \"num_topics\": 5, ...}}"
    )]
    req: AnalyzeDiscussionRequest,
) -> Result<AnalyzeDiscussionResponse> {
    SpaceApp::can_edit(role)?;

    if !req.params.validate() {
        return Err(Error::InvalidFormat);
    }

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    // Confirm the parent report exists + is Finish before letting the
    // user request a discussion analysis. Doing it here keeps the
    // stream Lambda pure (it can assume the row was admissible at
    // creation time).
    let report_id_str = report_id.to_string();
    let report_sk = EntityType::SpaceAnalyzeReport(report_id_str.clone());
    let report = SpaceAnalyzeReport::get(cli, &space_pk, Some(report_sk))
        .await?
        .ok_or(Error::NotFound("Analyze report not found".into()))?;
    if !matches!(report.status, AnalyzeReportStatus::Finish) {
        return Err(Error::NotFound(
            "Analyze report is not finished yet".into(),
        ));
    }

    let discussion_id_str = discussion_id.to_string();

    let row = SpaceAnalyzeDiscussionResult::new(
        space_pk,
        report_id_str.clone(),
        discussion_id_str.clone(),
        req.params,
    );
    let request_id = row.request_id.clone();

    row.create(cli).await.map_err(|e| {
        crate::error!("analyze_discussion: failed to enqueue request: {e}");
        e
    })?;

    Ok(AnalyzeDiscussionResponse {
        request_id,
        report_id: report_id_str,
        discussion_id: discussion_id_str,
    })
}
