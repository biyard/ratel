//! Detail-page analyze list — read-only projection of finished
//! `SpaceAnalyzeReport` rows for the space, hydrated with the
//! poll/quiz/follow aggregates from the matching
//! `SpaceAnalyzeReportResult` row.
//!
//! Drives the slash-command popup on the report detail editor.
//! Anything still in `InProgress` (no result row yet) is skipped —
//! the picker has nothing useful to insert into the document until
//! the aggregation pipeline finishes.

use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::discussion::SpacePost;
use crate::features::spaces::pages::apps::apps::analyzes::{
    AnalyzeFilterSource, AnalyzeReportFilter, AnalyzeReportStatus, FollowTargetAggregate,
    PollQuestionAggregate, QuizQuestionAggregate, SpaceAnalyzeDiscussionResult, SpaceAnalyzeReport,
    SpaceAnalyzeReportResult,
};
use crate::features::spaces::pages::report::models::SpaceReport;
use crate::features::spaces::pages::report::types::{
    ActionSource, Analyze, AnalyzeItem, ChartOption, CrossFilterChip, DiscussionData,
};
use crate::features::spaces::pages::report::*;
use std::collections::{HashMap, HashSet};

/// Walk the finished AnalyzeReport rows for a space and project them
/// into the `Analyze` shape the detail-page slash popup expects.
/// Skips rows whose result row is missing (typically `InProgress` —
/// the aggregation worker hasn't written the result yet).
#[get("/v3/spaces/{space_pk}/reports/analyzes", role: SpaceUserRole)]
pub async fn list_report_analyzes(
    space_pk: SpacePartition,
) -> Result<ListResponse<Analyze>> {
    SpaceReport::can_view(role)?;
    let space_partition: Partition = space_pk.into();

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    // Pre-fetch discussion (SpacePost) titles for the whole space once.
    // The discussion-analysis sk only carries ids, so we need a side
    // lookup to surface titles in the picker. Doing it once instead of
    // per-report keeps this from blowing up into N*M queries when the
    // space accumulates many reports.
    let post_titles: HashMap<String, String> =
        fetch_space_discussion_titles(dynamo, &space_partition).await?;

    // Page through every analyze report under the space. The detail
    // picker doesn't paginate so we surface them all at once; if this
    // ever grows large, switch to a bookmark-paginated DTO.
    let mut bookmark: Option<String> = None;
    let mut analyzes: Vec<Analyze> = Vec::new();
    loop {
        let mut opt = SpaceAnalyzeReport::opt()
            .sk(EntityType::SpaceAnalyzeReport(String::default()).to_string())
            .scan_index_forward(false)
            .limit(50);
        if let Some(bm) = bookmark.take() {
            opt = opt.bookmark(bm);
        }

        let (reports, next_bookmark) =
            SpaceAnalyzeReport::query(dynamo, space_partition.clone(), opt)
                .await
                .map_err(|e| {
                    crate::error!("list_report_analyzes: query failed: {e:?}");
                    crate::features::spaces::pages::report::types::SpaceReportError::ReportListFailed
                })?;

        for report in reports {
            if !matches!(report.status, AnalyzeReportStatus::Finish) {
                continue;
            }
            let report_id = match &report.sk {
                EntityType::SpaceAnalyzeReport(id) => id.clone(),
                _ => continue,
            };

            // Result row is optional — if it's gone (cleanup race,
            // dev-only state), surface the analyze with empty items
            // rather than dropping it.
            let result_sk = EntityType::SpaceAnalyzeReportResult(report_id.clone());
            let result = SpaceAnalyzeReportResult::get(dynamo, &space_partition, Some(result_sk))
                .await
                .map_err(|e| {
                    crate::error!("list_report_analyzes: result get failed: {e:?}");
                    crate::features::spaces::pages::report::types::SpaceReportError::ReportLoadFailed
                })?;

            // Latest finished discussion-analysis row per discussion
            // for THIS report. Skips rows still InProgress so the
            // picker only surfaces discussions that already have
            // LDA/TF-IDF/network payloads ready to embed.
            let discussion_items =
                latest_finished_discussions(dynamo, &space_partition, &report_id, &post_titles)
                    .await?;

            analyzes.push(project_analyze(
                report_id,
                &report,
                result.as_ref(),
                discussion_items,
            ));
        }

        match next_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(ListResponse {
        items: analyzes,
        bookmark: None,
    })
}

/// Map one SpaceAnalyzeReport (+ its result row + already-fetched
/// discussion items) into the detail-page `Analyze` shape. The
/// poll/quiz/follow buckets come from the result row's aggregates;
/// `discussion` is precomputed by `latest_finished_discussions` since
/// it requires a separate query against `SpaceAnalyzeDiscussionResult`.
#[cfg(feature = "server")]
fn project_analyze(
    id: String,
    report: &SpaceAnalyzeReport,
    result: Option<&SpaceAnalyzeReportResult>,
    discussion: Vec<AnalyzeItem>,
) -> Analyze {
    let (poll, quiz, follow) = result
        .map(|r| {
            (
                r.poll_aggregates
                    .iter()
                    .map(poll_to_item)
                    .collect::<Vec<_>>(),
                r.quiz_aggregates
                    .iter()
                    .map(quiz_to_item)
                    .collect::<Vec<_>>(),
                r.follow_aggregates
                    .iter()
                    .map(follow_to_item)
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_default();

    Analyze {
        id,
        name: report.name.clone(),
        respondents: report.respondent_count.max(0) as u32,
        filters: report.filters.iter().map(filter_to_chip).collect(),
        poll,
        quiz,
        discussion,
        follow,
    }
}

/// Pre-fetch every space discussion's title once so we can join it onto
/// `SpaceAnalyzeDiscussionResult` rows (which only carry the
/// discussion_id). Paginates through `SpacePost::find_by_space_ordered`
/// — same source used by the analyze detail page's sidebar list.
#[cfg(feature = "server")]
async fn fetch_space_discussion_titles(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<HashMap<String, String>> {
    let mut titles: HashMap<String, String> = HashMap::new();
    let mut bookmark: Option<String> = None;
    loop {
        let opt = SpacePost::opt_with_bookmark(bookmark.clone())
            .scan_index_forward(false)
            .limit(50);
        let (posts, next) = SpacePost::find_by_space_ordered(cli, space_pk.clone(), opt)
            .await
            .map_err(|e| {
                crate::error!("list_report_analyzes: discussion title query failed: {e:?}");
                crate::features::spaces::pages::report::types::SpaceReportError::ReportListFailed
            })?;
        for post in posts {
            let id = match post.sk {
                EntityType::SpacePost(id) => id,
                _ => continue,
            };
            titles.insert(id, post.title.trim().to_string());
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    Ok(titles)
}

/// For a given report, return the latest finished
/// `SpaceAnalyzeDiscussionResult` per discussion, projected into
/// `AnalyzeItem`. UUIDv7 in the sk gives lexicographic = chronological
/// order, so a `scan_index_forward(false)` sweep yields newest-first
/// and the first row seen per `discussion_id` is the latest. Rows still
/// `InProgress` are skipped — the picker only surfaces discussions whose
/// LDA/TF-IDF/network payload is ready to embed.
#[cfg(feature = "server")]
async fn latest_finished_discussions(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    report_id: &str,
    post_titles: &HashMap<String, String>,
) -> Result<Vec<AnalyzeItem>> {
    // The on-disk sk prefix is the SCREAMING_SNAKE form emitted by the
    // DynamoEnum derive — using the variant name `SpaceAnalyzeDiscussionResult`
    // verbatim here would silently miss every row.
    let sk_prefix = format!("SPACE_ANALYZE_DISCUSSION_RESULT#{}#", report_id);
    let opt = SpaceAnalyzeDiscussionResult::opt()
        .sk(sk_prefix)
        .scan_index_forward(false)
        .limit(100);
    let (rows, _) = SpaceAnalyzeDiscussionResult::query(cli, space_pk.clone(), opt)
        .await
        .map_err(|e| {
            crate::error!("list_report_analyzes: discussion result query failed: {e:?}");
            crate::features::spaces::pages::report::types::SpaceReportError::ReportListFailed
        })?;

    let mut seen: HashSet<String> = HashSet::new();
    let mut items: Vec<AnalyzeItem> = Vec::new();
    for row in rows {
        if !matches!(row.status, AnalyzeReportStatus::Finish) {
            continue;
        }
        if !seen.insert(row.discussion_id.clone()) {
            continue;
        }
        let title = post_titles
            .get(&row.discussion_id)
            .cloned()
            .unwrap_or_else(|| row.discussion_id.clone());
        items.push(AnalyzeItem {
            id: row.discussion_id.clone(),
            title,
            meta: format!(
                "{} 댓글 · 토픽 {} · 키워드 {}",
                row.analyzed_comment_count,
                row.topics.len(),
                row.tfidf_terms.len()
            ),
            options: Vec::new(),
            respondent_count: row.analyzed_comment_count.max(0) as u32,
            discussion_data: Some(DiscussionData {
                topics: row.topics,
                tfidf_terms: row.tfidf_terms,
                network_nodes: row.network_nodes,
                network_edges: row.network_edges,
            }),
            text_answers: Vec::new(),
        });
    }
    Ok(items)
}

#[cfg(feature = "server")]
fn poll_to_item(agg: &PollQuestionAggregate) -> AnalyzeItem {
    let options: Vec<ChartOption> = agg
        .options
        .iter()
        .map(|o| ChartOption {
            label: o.label.clone(),
            count: o.count,
        })
        .collect();
    // Subjective poll questions ship with no options but a populated
    // `text_answers` list — surface that here so the picker / report
    // can render it as a text-list chart.
    let meta = if options.is_empty() && !agg.text_answers.is_empty() {
        format!("{} · 주관식 · {} 응답", agg.poll_title, agg.text_answers.len())
    } else {
        format!("{} · {} 응답", agg.poll_title, agg.respondent_count)
    };
    AnalyzeItem {
        id: format!("{}#{}", agg.poll_id, agg.question_idx),
        title: if agg.question_title.is_empty() {
            format!("Q{}", agg.question_idx + 1)
        } else {
            agg.question_title.clone()
        },
        meta,
        options,
        respondent_count: agg.respondent_count,
        discussion_data: None,
        text_answers: agg.text_answers.clone(),
    }
}

#[cfg(feature = "server")]
fn quiz_to_item(agg: &QuizQuestionAggregate) -> AnalyzeItem {
    let options: Vec<ChartOption> = agg
        .options
        .iter()
        .map(|o| ChartOption {
            label: o.label.clone(),
            count: o.count,
        })
        .collect();
    let meta = if options.is_empty() && !agg.text_answers.is_empty() {
        format!("{} · 주관식 · {} 응답", agg.quiz_title, agg.text_answers.len())
    } else {
        format!(
            "{} · 정답 {}/{}",
            agg.quiz_title, agg.correct_count, agg.respondent_count
        )
    };
    AnalyzeItem {
        id: format!("{}#{}", agg.quiz_id, agg.question_idx),
        title: if agg.question_title.is_empty() {
            format!("Q{}", agg.question_idx + 1)
        } else {
            agg.question_title.clone()
        },
        meta,
        options,
        respondent_count: agg.respondent_count,
        discussion_data: None,
        text_answers: agg.text_answers.clone(),
    }
}

#[cfg(feature = "server")]
fn follow_to_item(agg: &FollowTargetAggregate) -> AnalyzeItem {
    // Follow items don't have a multi-option tally — synthesize a
    // two-slice payload (followers vs everyone else) so Pie /
    // Bar / Table charts get something meaningful to render.
    let label = if agg.display_name.is_empty() {
        format!("@{}", agg.username)
    } else {
        agg.display_name.clone()
    };
    AnalyzeItem {
        id: agg.user_pk.clone(),
        title: label.clone(),
        meta: format!("{} 매칭 팔로워", agg.count),
        options: vec![ChartOption {
            label,
            count: agg.count,
        }],
        respondent_count: agg.count,
        discussion_data: None,
        text_answers: Vec::new(),
    }
}

#[cfg(feature = "server")]
fn filter_to_chip(f: &AnalyzeReportFilter) -> CrossFilterChip {
    CrossFilterChip {
        source: map_source(f.source),
        label: if f.label.is_empty() {
            f.option_text.clone()
        } else {
            f.label.clone()
        },
    }
}

#[cfg(feature = "server")]
fn map_source(s: AnalyzeFilterSource) -> ActionSource {
    match s {
        AnalyzeFilterSource::Poll => ActionSource::Poll,
        AnalyzeFilterSource::Quiz => ActionSource::Quiz,
        AnalyzeFilterSource::Discussion => ActionSource::Discussion,
        AnalyzeFilterSource::Follow => ActionSource::Follow,
    }
}
