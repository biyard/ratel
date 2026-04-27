use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

/// Controller for the Analyze REPORT detail arena.
///
/// The detail page is mostly read-only mock content (Phase 3 ships the
/// arena split-panel result view). The only Dioxus-owned state is the
/// resolved `AnalyzeReport` (looked up by `report_id` from `mock_reports()`,
/// falling back to the first mock when nothing matches).
///
/// Sidebar item click → which panel is active is **JS-owned**: `script.js`
/// reads `data-target-panel` on `.sb-item`, sets `data-active="true"` on
/// the matching `<section class="panel">`, and toggles `aria-selected` on
/// sb-items. Same goes for sb-group collapse, bar-row / tfidf-row /
/// topic-table filter highlights — JS owns those too. Keeping that
/// state out of Rust signals avoids round-tripping through the rerender
/// loop, matches the home/script.js pattern, and lines up with the
/// `pages/index/action_dashboard` carousel precedent.
#[derive(Clone, Copy)]
pub struct UseAnalyzeReportDetail {
    pub report: Signal<AnalyzeReport>,
}

#[track_caller]
pub fn use_analyze_report_detail(
    report_id: ReadSignal<String>,
) -> std::result::Result<UseAnalyzeReportDetail, RenderError> {
    if let Some(ctx) = try_use_context::<UseAnalyzeReportDetail>() {
        return Ok(ctx);
    }

    // Mock-only resolution: look up by id, fall back to the first
    // seeded report when nothing matches. Phase 3 is a visual port,
    // not a data layer.
    let report = use_signal(move || {
        let rid = report_id();
        let reports = mock_reports();
        reports
            .iter()
            .find(|r| r.id == rid)
            .cloned()
            .or_else(|| reports.into_iter().next())
            .unwrap_or_else(|| AnalyzeReport {
                id: rid,
                name: String::new(),
                status: AnalyzeReportStatus::Done,
                created_at: String::new(),
                created_at_time: String::new(),
                filters: Vec::new(),
            })
    });

    Ok(use_context_provider(|| UseAnalyzeReportDetail { report }))
}
