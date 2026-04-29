use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::discussion::SpacePost;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeDiscussionItem {
    pub discussion_id: SpacePostEntityType,
    pub title: String,
    /// Total comments on the discussion (unfiltered). Kept for the
    /// list page that doesn't apply cross filters.
    /// Defaults to 0 when omitted by older clients.
    #[serde(default)]
    pub comment_count: i64,

    /// Comments authored by users in the report's cross-filter
    /// matched set. Equals `comment_count` when the report has no
    /// filters. Populated by `get_analyze_report`; the bare
    /// `list_analyze_discussions` endpoint leaves this at 0.
    #[serde(default)]
    pub matched_comment_count: i64,

    /// Distinct authors (from the matched set) who commented on the
    /// discussion. Same fallback semantics as
    /// `matched_comment_count` — 0 from the bare list endpoint,
    /// populated by `get_analyze_report`.
    #[serde(default)]
    pub matched_participant_count: i64,
}

#[get("/api/spaces/{space_id}/apps/analyzes/discussions?bookmark", role: SpaceUserRole)]
pub async fn list_analyze_discussions(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<AnalyzeDiscussionItem>> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let opt = SpacePost::opt_with_bookmark(bookmark)
        .scan_index_forward(false)
        .limit(20);
    let (posts, bookmark) = SpacePost::find_by_space_ordered(cli, space_pk, opt).await?;

    let items = posts
        .into_iter()
        .map(|post| AnalyzeDiscussionItem {
            discussion_id: post.sk.clone().into(),
            title: post.title.trim().to_string(),
            comment_count: post.comments,
            matched_comment_count: 0,
            matched_participant_count: 0,
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
