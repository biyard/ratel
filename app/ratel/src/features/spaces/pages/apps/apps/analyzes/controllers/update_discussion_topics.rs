//! PATCH the LDA topic names on an existing discussion-analysis row.
//!
//! Lets the user rename auto-generated `토픽_N` labels to something
//! meaningful (e.g. "정부 권한 강화"). Body is the full
//! `Vec<TopicRow>` for the row — same shape as the analysis result
//! returns — so the client just submits whatever the table currently
//! shows. Keywords are preserved verbatim from the original row;
//! only `topic` (the label) ends up changed.
//!
//! Auth: same `SpaceUserRole` gate as the rest of the analyze app.
//! Mutation is a single `UpdateItem` — no transactional concerns.

use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateDiscussionTopicsRequest {
    pub topics: Vec<TopicRow>,
}

#[mcp_tool(
    name = "update_discussion_topics",
    description = "Rename the LDA topic labels on an existing discussion-analysis row. Submit the full Vec<TopicRow> for the row; only `topic` (label) ends up changed. Requires creator role."
)]
#[post(
    "/api/spaces/{space_id}/apps/analyzes/reports/{report_id}/discussions/{discussion_id}/results/{request_id}/topics",
    role: SpaceUserRole
)]
pub async fn update_discussion_topics(
    #[mcp(description = "Space partition key")] space_id: SpacePartition,
    #[mcp(description = "Analyze report id")] report_id: SpaceAnalyzeReportEntityType,
    #[mcp(description = "Discussion (post) partition key")] discussion_id: FeedPartition,
    #[mcp(description = "Discussion analysis request id (UUIDv7) returned by analyze_discussion")]
    request_id: String,
    #[mcp(
        description = "Topics payload as JSON, e.g. {\"topics\": [{\"topic\": \"...\", \"keywords\": [...], \"weight\": ...}, ...]}"
    )]
    req: UpdateDiscussionTopicsRequest,
) -> Result<()> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    // Sk pattern: SPACE_ANALYZE_DISCUSSION_RESULT#{report_id}#{discussion_id}#{request_id}
    let sk = EntityType::SpaceAnalyzeDiscussionResult(
        report_id.to_string(),
        format!("{}#{}", discussion_id, request_id),
    );

    SpaceAnalyzeDiscussionResult::updater(space_pk, sk)
        .with_topics(req.topics)
        .with_updated_at(crate::common::utils::time::get_now_timestamp_millis())
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("update_discussion_topics: {e}");
            Error::Internal
        })?;

    Ok(())
}
