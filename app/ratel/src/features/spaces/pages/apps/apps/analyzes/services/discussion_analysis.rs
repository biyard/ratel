//! User-triggered discussion analysis Lambda handler.
//!
//! Fired by the DDB stream INSERT pipe pinned to
//! `SpaceAnalyzeDiscussionResult` rows. Picks up the request, loads
//! the matched-user comment corpus on the target discussion, runs
//! the lindera + Gibbs LDA + TF-IDF + text-network pipeline, then
//! overwrites the SAME row with results and `status=Finish`. The
//! INSERT-only filter on the pipe keeps this update from
//! re-triggering the handler.
//!
//! Failure modes:
//! - Parent report missing or filters empty → write empty result with
//!   `status=Failed` so the UI can surface a clear "no data" state.
//! - Tokeniser blow-up on a single comment → that document drops out;
//!   the rest of the corpus still produces a result.
//! - Empty corpus after preprocessing → status=Finish with all result
//!   arrays empty; the UI shows the empty-state per panel.

use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use std::collections::HashSet;

/// Outer wrapper: catches every failure path inside the analysis
/// pipeline and stamps `status=Failed` on the row before bubbling
/// the error up. Without this, a transient DDB/CompUTE failure would
/// leave the row stuck on `InProgress` forever — the UI would keep
/// showing "분석 진행 중" indefinitely with no recovery path short
/// of manually editing the row in the DDB console.
pub async fn process_discussion_analysis(
    cli: &aws_sdk_dynamodb::Client,
    row: &SpaceAnalyzeDiscussionResult,
) -> Result<()> {
    match try_process_discussion_analysis(cli, row).await {
        Ok(()) => Ok(()),
        Err(e) => {
            crate::error!(
                "discussion_analysis failed for sk={} report={} discussion={}: {e}",
                row.sk,
                row.report_id,
                row.discussion_id,
            );
            // Best-effort: try to flip status to Failed. If this also
            // fails (e.g. DDB outage) the row stays InProgress; we
            // accept that — operationally a rare double-failure case.
            let _ = mark_failed(cli, row).await;
            Err(e)
        }
    }
}

async fn try_process_discussion_analysis(
    cli: &aws_sdk_dynamodb::Client,
    row: &SpaceAnalyzeDiscussionResult,
) -> Result<()> {
    let space_pk = row.pk.clone();

    // 1. Load parent report — fail-fast if missing.
    let report_sk = EntityType::SpaceAnalyzeReport(row.report_id.clone());
    let report = match SpaceAnalyzeReport::get(cli, &space_pk, Some(report_sk)).await? {
        Some(r) => r,
        None => {
            crate::error!(
                "discussion_analysis: parent report {} missing — marking failed",
                row.report_id
            );
            return mark_failed(cli, row).await;
        }
    };

    // 2. Compute matched-user set from the report's filters. Same
    //    semantics as preview / auto-analysis — empty filter list
    //    means "all space participants".
    let matched_users: HashSet<String> = if report.filters.is_empty() {
        services::intersection::list_participant_user_pks(cli, &space_pk)
            .await?
            .into_iter()
            .map(|p| p.to_string())
            .collect()
    } else {
        let (set, _, _) =
            services::intersection::intersect_filters(cli, &space_pk, &report.filters).await?;
        set
    };

    // 3. Pull every comment on the target discussion authored by a
    //    matched user.
    let comments = load_matched_comments(cli, &row.discussion_id, &matched_users).await?;
    let comment_count = comments.len() as i64;

    // 4. Tokenise the corpus once — every analyser shares the result.
    let extra_stopwords: HashSet<String> = row
        .params
        .excluded_keywords
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    let token_docs: Vec<Vec<String>> = comments
        .iter()
        .map(|c| services::text_pipeline::preprocess::preprocess_korean_nouns(c, &extra_stopwords))
        .collect();

    // 5. Run all three analyses. They share the same token corpus.
    let lda_cfg = services::text_pipeline::lda::LdaConfigV1 {
        num_topics: row.params.num_topics,
        top_n: 10,
        ..Default::default()
    };
    let topics = services::text_pipeline::lda::run_lda(token_docs.clone(), lda_cfg);
    let tfidf_terms =
        services::text_pipeline::tfidf::run_tfidf(&token_docs, row.params.top_n_tfidf);
    let (network_nodes, network_edges) =
        services::text_pipeline::text_network::run_text_network(&token_docs, row.params.top_n_network);

    // 6. Overwrite the row with results. updater's `with_*` setters
    //    flip every field in one transactional UpdateItem.
    SpaceAnalyzeDiscussionResult::updater(row.pk.clone(), row.sk.clone())
        .with_status(AnalyzeReportStatus::Finish)
        .with_analyzed_comment_count(comment_count)
        .with_topics(topics)
        .with_tfidf_terms(tfidf_terms)
        .with_network_nodes(network_nodes)
        .with_network_edges(network_edges)
        .with_updated_at(get_now_timestamp_millis())
        .execute(cli)
        .await?;

    Ok(())
}

async fn mark_failed(
    cli: &aws_sdk_dynamodb::Client,
    row: &SpaceAnalyzeDiscussionResult,
) -> Result<()> {
    SpaceAnalyzeDiscussionResult::updater(row.pk.clone(), row.sk.clone())
        .with_status(AnalyzeReportStatus::Failed)
        .with_updated_at(get_now_timestamp_millis())
        .execute(cli)
        .await?;
    Ok(())
}

async fn load_matched_comments(
    cli: &aws_sdk_dynamodb::Client,
    discussion_id: &str,
    matched_users: &HashSet<String>,
) -> Result<Vec<String>> {
    let post_pk = Partition::SpacePost(discussion_id.to_string());
    let mut comments: Vec<String> = Vec::new();

    // Pulls top-level + reply bodies. `iter_post_comments` covers
    // both sk prefixes (`SPACE_POST_COMMENT#` and
    // `SPACE_POST_COMMENT_REPLY#`) so the LDA / TF-IDF / network
    // pipeline ingests every text the matched users contributed —
    // not just root comments.
    services::intersection::iter_post_comments(cli, post_pk, |row| {
        if matched_users.contains(&row.author_pk.to_string()) && !row.content.trim().is_empty() {
            comments.push(row.content);
        }
    })
    .await?;

    Ok(comments)
}
