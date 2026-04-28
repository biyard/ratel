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

use crate::features::spaces::pages::actions::actions::poll::controllers::{
    get_poll, get_poll_result,
};
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::apps::panels::list_panels;
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

    /// Per-poll Excel export. Loads raw poll + per-user result + panel
    /// data on demand, runs the legacy builder, and pipes through the
    /// JS download bridge. Input is the active poll id (sidebar's
    /// selected poll).
    pub handle_export_excel: Action<(String,), ()>,
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

    let tr_for_excel: SpaceAnalyzesAppTranslate = use_translate();
    let download_started_text = tr_for_excel.download_started.to_string();
    let handle_export_excel = use_action(move |poll_id: String| {
        let tr = tr_for_excel.clone();
        let download_started = download_started_text.clone();
        async move {
            let mut toast = toast;
            let sid = space_id();
            let rid = report_id();
            let pid: SpacePollEntityType = poll_id.into();
            let poll_data = match get_poll(sid.clone(), pid.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    crate::error!("export_excel get_poll failed: {err}");
                    toast.error(err);
                    return Ok::<(), crate::common::Error>(());
                }
            };
            let mut result_data = match get_poll_result(sid.clone(), pid).await {
                Ok(v) => v,
                Err(err) => {
                    crate::error!("export_excel get_poll_result failed: {err}");
                    toast.error(err);
                    return Ok::<(), crate::common::Error>(());
                }
            };
            let panels_data = match list_panels(sid.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    crate::error!("export_excel list_panels failed: {err}");
                    toast.error(err);
                    return Ok::<(), crate::common::Error>(());
                }
            };

            // Cross-filter the per-user answer lists.
            //
            // Empty filter list → no retain runs, the workbook contains
            // every respondent (= the same "전체" semantics the page
            // shows when no chip is selected).
            //
            // Non-empty filters → call `get_matched_users` (server-side
            // intersect across Poll/Quiz/Discussion/Follow filter
            // sources) and keep only the answers whose user pk lands
            // in that set.
            let report = detail.read().clone().report;
            let has_filters = !report.filters.is_empty();
            if has_filters {
                let report_id_typed: SpaceAnalyzeReportEntityType = rid.into();
                let matched: std::collections::HashSet<String> =
                    match get_matched_users(sid.clone(), report_id_typed).await {
                        Ok(v) => v.into_iter().collect(),
                        Err(err) => {
                            crate::error!("export_excel get_matched_users failed: {err}");
                            toast.error(err);
                            return Ok::<(), crate::common::Error>(());
                        }
                    };
                // `a.pk` is `Partition::SpacePollUserAnswer(...)` —
                // a wrapper variant, NOT the user partition. The
                // server-side intersection inserts the actual
                // `Partition::User(...)` from `row.user_pk` (with a
                // fallback to `row.pk` only when `user_pk` is None,
                // which the row builders always populate). So the
                // retain must compare against `a.user_pk`, not
                // `a.pk`, otherwise every answer is filtered out
                // because the wrapper format never matches.
                result_data.sample_answers.retain(|a| {
                    a.user_pk
                        .as_ref()
                        .is_some_and(|p| matched.contains(&p.to_string()))
                });
                result_data.final_answers.retain(|a| {
                    a.user_pk
                        .as_ref()
                        .is_some_and(|p| matched.contains(&p.to_string()))
                });
            }

            let excel = build_excel_data(&poll_data, &panels_data, &result_data, &tr);
            match download_analyze_excel(DownloadAnalyzeExcelRequest {
                file_name: build_excel_file_name(&sid),
                sheet_name: "Responses".to_string(),
                rows: excel.rows,
                merges: excel.merges,
            })
            .await
            {
                Ok(_) => {
                    toast.info(download_started);
                }
                Err(err) => {
                    toast.error(err);
                }
            }
            Ok::<(), crate::common::Error>(())
        }
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
        handle_export_excel,
    }))
}
