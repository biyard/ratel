//! Records-page controller. Loads the saved report (for chip metadata
//! to drive the filter-tab strip) and an `InfiniteQuery` of hydrated
//! `AnalyzeRecordRow`s for the currently selected `filter_idx`.
//!
//! `selected_filter` is a `Signal<Option<u32>>`: `None` means "no
//! filter picked yet" (initial state, shows hint), `Some(idx)` switches
//! the records query and triggers a refresh through the loader's own
//! reactive subscription on `selected_filter`.

use crate::common::ListResponse;
use crate::common::hooks::{InfiniteQuery, use_infinite_query};
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseAnalyzeRecords {
    pub space_id: ReadSignal<SpacePartition>,
    pub report_id: ReadSignal<String>,

    /// The saved report — needed for the filter-chip tab strip.
    /// Reuses the existing `get_analyze_report` endpoint so we don't
    /// add a server fn just to fetch metadata.
    pub report: Loader<GetAnalyzeReportResponse>,

    /// `None` until the user picks a chip; `Some(idx)` while a tab is
    /// active. Drives both the records query and the highlighted
    /// chip in the strip.
    pub selected_filter: Signal<Option<u32>>,

    pub records: InfiniteQuery<String, AnalyzeRecordRow, ListResponse<AnalyzeRecordRow>>,
}

#[track_caller]
pub fn use_analyze_records(
    space_id: ReadSignal<SpacePartition>,
    report_id: ReadSignal<String>,
) -> std::result::Result<UseAnalyzeRecords, RenderError> {
    if let Some(ctx) = try_use_context::<UseAnalyzeRecords>() {
        return Ok(ctx);
    }

    let report = use_loader(move || {
        let rid = report_id.read().clone();
        let sid = space_id();
        async move {
            let report_id_typed: SpaceAnalyzeReportEntityType = rid.into();
            get_analyze_report(sid, report_id_typed).await
        }
    })?;

    let selected_filter = use_signal::<Option<u32>>(|| None);

    // Read `selected_filter` in the OUTER closure so the loader's
    // ReactiveContext records a subscription on it — otherwise reads
    // inside `async move` happen post-poll and are invisible to the
    // reactive layer (see `feedback_use_loader_ssr_subscriptions`).
    let records = use_infinite_query(move |bookmark| {
        let filter_idx = *selected_filter.read();
        let rid = report_id.read().clone();
        let sid = space_id();
        async move {
            // No active filter → empty page. Pagination falls through
            // because `bookmark = None` plus zero items signals "done"
            // to InfiniteQuery.
            let filter_idx = match filter_idx {
                Some(v) => v,
                None => return Ok(ListResponse::default()),
            };
            let report_id_typed: SpaceAnalyzeReportEntityType = rid.into();
            list_analyze_records(sid, report_id_typed, Some(filter_idx), bookmark).await
        }
    })?;

    Ok(use_context_provider(|| UseAnalyzeRecords {
        space_id,
        report_id,
        report,
        selected_filter,
        records,
    }))
}
