use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

/// Controller for the Analyze LIST arena.
///
/// Bundles the saved-report list (mock for now). Carousel state
/// (active index, scroll position, prev/next disabled) lives in JS
/// via native scroll-snap — see `views/home/script.js` and the
/// existing `action_dashboard` carousel for the same pattern. The
/// hook only owns the data Dioxus renders; ephemeral UI state stays
/// out of Rust signals.
#[derive(Clone, Copy)]
pub struct UseAnalyzeReports {
    pub reports: Signal<Vec<AnalyzeReport>>,
}

#[track_caller]
pub fn use_analyze_reports() -> std::result::Result<UseAnalyzeReports, RenderError> {
    if let Some(ctx) = try_use_context::<UseAnalyzeReports>() {
        return Ok(ctx);
    }
    let reports = use_signal(mock_reports);
    Ok(use_context_provider(|| UseAnalyzeReports { reports }))
}
