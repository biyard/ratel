//! Detail-page controller. Loads the combined report payload
//! (`get_analyze_report`) and exposes:
//!
//! - `report`: metadata + filters + status
//! - `result`: poll / quiz / follow aggregates (None during loading)
//! - `discussions`: sidebar candidate list
//! - `selected_discussion`: which discussion the sidebar has active
//! - `discussion_results`: latest discussion analysis per (report, discussion)
//! - `params`: live form state for the 분석 설정 card
//! - `handle_run_discussion`: action that POSTs analyze_discussion +
//!   refreshes the per-discussion history loader

use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseAnalyzeReportDetail {
    pub report_id: ReadSignal<String>,
    pub space_id: ReadSignal<SpacePartition>,

    pub detail: Loader<GetAnalyzeReportResponse>,

    /// Active poll the sidebar has highlighted (and the panel
    /// filters to). `None` falls back to "first poll in
    /// poll_aggregates" so the panel always renders something
    /// meaningful when the user hasn't picked yet.
    pub selected_poll: Signal<Option<String>>,
    pub selected_quiz: Signal<Option<String>>,

    /// Currently-active sidebar discussion id (plain string).
    /// `None` when nothing's selected yet.
    pub selected_discussion: Signal<Option<String>>,

    /// Per-discussion history loader. Re-fires whenever
    /// `selected_discussion` changes; caller reads `.items[0]` for
    /// the latest run.
    pub discussion_results:
        Loader<crate::common::ListResponse<SpaceAnalyzeDiscussionResult>>,

    /// Live form state for the 분석 설정 card.
    pub params: Signal<DiscussionAnalysisParams>,

    pub handle_run_discussion: Action<(), ()>,
}

#[track_caller]
pub fn use_analyze_report_detail(
    report_id: ReadSignal<String>,
    space_id: ReadSignal<SpacePartition>,
) -> std::result::Result<UseAnalyzeReportDetail, RenderError> {
    if let Some(ctx) = try_use_context::<UseAnalyzeReportDetail>() {
        return Ok(ctx);
    }

    let detail = use_loader(move || {
        let rid = report_id();
        let sid = space_id();
        async move {
            let report_id_typed: SpaceAnalyzeReportEntityType = rid.into();
            get_analyze_report(sid, report_id_typed).await
        }
    })?;

    let selected_poll = use_signal::<Option<String>>(|| None);
    let selected_quiz = use_signal::<Option<String>>(|| None);
    let selected_discussion = use_signal::<Option<String>>(|| None);

    // Resolve the active discussion the loader should query against:
    // explicit user selection wins, otherwise fall back to the first
    // discussion exposed by the detail payload. Without this fallback
    // the loader silently returned empty on first page load —
    // sidebar shows the first discussion visually selected but the
    // signal was still `None`, so InProgress rows weren't surfaced
    // until the user manually clicked the sidebar item.
    let discussion_results = use_loader(move || {
        let rid = report_id();
        let sid = space_id();
        let explicit = selected_discussion.read().clone();
        let detail_snapshot = detail.read().clone();
        let fallback = detail_snapshot
            .discussions
            .first()
            .map(|d| d.discussion_id.to_string());
        let did = explicit.or(fallback);
        async move {
            let did = match did {
                Some(d) if !d.is_empty() => d,
                _ => {
                    return Ok::<crate::common::ListResponse<SpaceAnalyzeDiscussionResult>, _>(
                        crate::common::ListResponse {
                            items: Vec::new(),
                            bookmark: None,
                        },
                    );
                }
            };
            let report_id_typed: SpaceAnalyzeReportEntityType = rid.into();
            let discussion_id_typed: FeedPartition = did.into();
            list_analyze_discussion_results(sid, report_id_typed, discussion_id_typed, None).await
        }
    })?;

    let params = use_signal(|| DiscussionAnalysisParams {
        num_topics: 10,
        top_n_tfidf: 20,
        top_n_network: 15,
        excluded_keywords: Vec::new(),
    });

    let mut toast = use_toast();
    let mut discussion_results_handle = discussion_results;

    let handle_run_discussion = use_action(move || async move {
        let explicit = selected_discussion.read().clone();
        let fallback = detail
            .read()
            .clone()
            .discussions
            .first()
            .map(|d| d.discussion_id.to_string());
        let did = match explicit.or(fallback) {
            Some(d) if !d.is_empty() => d,
            _ => return Ok::<(), crate::common::Error>(()),
        };
        let p = params.read().clone();
        let report_id_typed: SpaceAnalyzeReportEntityType = report_id().into();
        let discussion_id_typed: FeedPartition = did.into();
        match analyze_discussion(
            space_id(),
            report_id_typed,
            discussion_id_typed,
            AnalyzeDiscussionRequest { params: p },
        )
        .await
        {
            Ok(_) => {
                // Stream-driven Lambda will overwrite the row with the
                // result. Refetch once so the UI immediately shows the
                // freshly-queued row in `InProgress` state, then the
                // detail page polls / refetches on its own cadence.
                discussion_results_handle.restart();
            }
            Err(err) => {
                crate::error!("analyze_discussion failed: {err}");
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseAnalyzeReportDetail {
        report_id,
        space_id,
        detail,
        selected_poll,
        selected_quiz,
        selected_discussion,
        discussion_results,
        params,
        handle_run_discussion,
    }))
}
