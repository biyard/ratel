use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;
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
                status: AnalyzeReportStatus::Finish,
                created_at: 0,
                filters: Vec::new(),
            })
    });

    Ok(use_context_provider(|| UseAnalyzeReportDetail { report }))
}
