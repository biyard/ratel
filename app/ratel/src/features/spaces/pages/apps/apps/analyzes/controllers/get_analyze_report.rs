//! Combined detail-page payload for one saved AnalyzeReport.
//!
//! Returns the report's metadata, the auto-computed
//! poll/quiz/follow aggregates, AND the discussion candidates the
//! sidebar lists. Does NOT include discussion analysis results — those
//! live on `SpaceAnalyzeDiscussionResult` rows queried separately
//! once the user picks a discussion in the sidebar.
//!
//! One trip is enough for the initial page render: detail page rarely
//! needs partial data, and combining the shapes lets the client cache
//! / prefetch as a single resource.

use crate::common::ListResponse;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct GetAnalyzeReportResponse {
    pub report: AnalyzeReport,

    /// Auto-aggregation result. `None` while
    /// `report.status != Finish`. Always present for callers that
    /// have already passed the list-page status gate, but kept
    /// optional so the type doesn't lie about lifecycle.
    pub result: Option<AnalyzeReportResultPayload>,

    /// Discussions the sidebar should list. Currently mirrors the
    /// space-level discussion list. The discussion analysis subview
    /// will lazily hydrate per-discussion results using the
    /// `list_analyze_discussion_results` endpoint when the user
    /// picks one.
    pub discussions: Vec<AnalyzeDiscussionItem>,
}

/// Plain serialisable mirror of `SpaceAnalyzeReportResult`'s analysis
/// fields. The DynamoEntity itself can't cross the wire as-is because
/// `pk` / `sk` aren't useful to the client.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeReportResultPayload {
    pub respondent_count: i64,
    pub poll_aggregates: Vec<PollQuestionAggregate>,
    pub quiz_aggregates: Vec<QuizQuestionAggregate>,
    pub follow_aggregates: Vec<FollowTargetAggregate>,
}

#[get(
    "/api/spaces/{space_id}/apps/analyzes/reports/{report_id}",
    role: SpaceUserRole
)]
pub async fn get_analyze_report(
    space_id: SpacePartition,
    report_id: SpaceAnalyzeReportEntityType,
) -> Result<GetAnalyzeReportResponse> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    let report_id_str = report_id.to_string();

    // 1. Report metadata.
    let report_sk = EntityType::SpaceAnalyzeReport(report_id_str.clone());
    let report_row = SpaceAnalyzeReport::get(cli, &space_pk, Some(report_sk))
        .await?
        .ok_or(Error::NotFound("Analyze report not found".into()))?;
    let report_dto = AnalyzeReport {
        id: report_id_str.clone(),
        name: report_row.name.clone(),
        status: report_row.status,
        created_at: report_row.created_at,
        filters: report_row.filters.clone(),
    };

    // 2. Result row (1:1 with report). Absent while status is
    //    InProgress; the list page gates navigation on Finish so this
    //    should normally exist.
    let result_sk = EntityType::SpaceAnalyzeReportResult(report_id_str.clone());
    let result_row = SpaceAnalyzeReportResult::get(cli, &space_pk, Some(result_sk)).await?;
    let result_payload = result_row.map(|r| AnalyzeReportResultPayload {
        respondent_count: r.respondent_count,
        poll_aggregates: r.poll_aggregates,
        quiz_aggregates: r.quiz_aggregates,
        follow_aggregates: r.follow_aggregates,
    });

    // 3. Sidebar discussion list — re-uses `list_analyze_discussions`'s
    //    payload shape for symmetry with the create wizard's picker.
    let mut discussions = fetch_discussion_candidates(cli, &space_pk).await?;

    // 4. Decorate each discussion with cross-filter aware counts so
    //    the sidebar can show "X 댓글 · Y명 참여" for the matched
    //    audience instead of the raw post-level numbers. Empty
    //    filter list = "everyone" (matched_count == comment_count).
    let matched: std::collections::HashSet<String> = if report_row.filters.is_empty() {
        services::intersection::list_participant_user_pks(cli, &space_pk)
            .await?
            .into_iter()
            .map(|p| p.to_string())
            .collect()
    } else {
        let (set, _, _) =
            services::intersection::intersect_filters(cli, &space_pk, &report_row.filters).await?;
        set
    };
    decorate_with_matched_counts(cli, &mut discussions, &matched).await?;

    Ok(GetAnalyzeReportResponse {
        report: report_dto,
        result: result_payload,
        discussions,
    })
}

/// Walk each discussion's comments once and tally the slice authored
/// by users in `matched`. We're explicitly not concurrent here —
/// space-wide discussion counts are typically a handful of posts,
/// each with at most a few hundred comments, and serial keeps the
/// memory + DDB-RCU footprint predictable.
///
/// Counts top-level comments AND replies — `iter_post_comments`
/// walks both prefixes for us.
#[cfg(feature = "server")]
async fn decorate_with_matched_counts(
    cli: &aws_sdk_dynamodb::Client,
    items: &mut [AnalyzeDiscussionItem],
    matched: &std::collections::HashSet<String>,
) -> Result<()> {
    use std::collections::HashSet;

    for item in items.iter_mut() {
        let post_pk: Partition = Partition::SpacePost(item.discussion_id.to_string());
        let mut comment_count: i64 = 0;
        let mut participants: HashSet<String> = HashSet::new();
        services::intersection::iter_post_comments(cli, post_pk, |row| {
            let author = row.author_pk.to_string();
            if matched.contains(&author) {
                comment_count += 1;
                participants.insert(author);
            }
        })
        .await?;
        item.matched_comment_count = comment_count;
        item.matched_participant_count = participants.len() as i64;
    }
    Ok(())
}

#[cfg(feature = "server")]
async fn fetch_discussion_candidates(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Vec<AnalyzeDiscussionItem>> {
    use crate::features::spaces::pages::actions::actions::discussion::SpacePost;

    // Mirrors `list_analyze_discussions`: paginate through every
    // `SpacePost` for the space (the discussion entity uses GSI3 to
    // be queryable by space_pk + updated_at). Older paths via
    // `SpaceAction::query` returned 0 because Discussion's canonical
    // row lives on `SpacePost`, not on the generic `SpaceAction` sk.
    let mut bookmark: Option<String> = None;
    let mut items: Vec<AnalyzeDiscussionItem> = Vec::new();
    loop {
        let opt = SpacePost::opt_with_bookmark(bookmark.clone())
            .scan_index_forward(false)
            .limit(50);
        let (posts, next) =
            SpacePost::find_by_space_ordered(cli, space_pk.clone(), opt).await?;
        for post in posts {
            items.push(AnalyzeDiscussionItem {
                discussion_id: post.sk.clone().into(),
                title: post.title.trim().to_string(),
                comment_count: post.comments,
                matched_comment_count: 0,
                matched_participant_count: 0,
            });
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(items)
}

/// History of discussion analyses for one (report, discussion). The
/// detail page shows the latest result (head of this list) and
/// optionally exposes the rest behind a "이전 분석" affordance. Most
/// recent first thanks to UUIDv7's lexicographic sort.
#[get(
    "/api/spaces/{space_id}/apps/analyzes/reports/{report_id}/discussions/{discussion_id}/results?bookmark",
    role: SpaceUserRole
)]
pub async fn list_analyze_discussion_results(
    space_id: SpacePartition,
    report_id: SpaceAnalyzeReportEntityType,
    discussion_id: FeedPartition,
    bookmark: Option<String>,
) -> Result<ListResponse<SpaceAnalyzeDiscussionResult>> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    // Sk pattern: SPACE_ANALYZE_DISCUSSION_RESULT#{report_id}#{discussion_id}#{uuid}
    // begins_with prefix groups every history row for this pair.
    // Use the canonical SCREAMING_SNAKE prefix that DynamoEnum
    // emits — building it from the variant name (`PascalCase`) here
    // would silently miss every row. We confirmed the on-disk shape
    // by reading a row in the DDB console: it really is uppercase
    // with underscores, matching the auto-analysis writes.
    let sk_prefix = format!(
        "SPACE_ANALYZE_DISCUSSION_RESULT#{}#{}#",
        report_id, discussion_id
    );
    let mut opt = SpaceAnalyzeDiscussionResult::opt()
        .sk(sk_prefix)
        .scan_index_forward(false)
        .limit(20);
    if let Some(b) = bookmark {
        opt = opt.bookmark(b);
    }
    let (items, bookmark) = SpaceAnalyzeDiscussionResult::query(cli, space_pk, opt).await?;
    Ok(ListResponse { items, bookmark })
}
