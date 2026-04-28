use crate::common::ListResponse;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

#[derive(Clone, Copy)]
pub struct UseAnalyzeReports {
    pub space_id: ReadSignal<SpacePartition>,
    pub reports: Loader<ListResponse<AnalyzeReport>>,
}

#[track_caller]
pub fn use_analyze_reports(
    space_id: ReadSignal<SpacePartition>,
) -> std::result::Result<UseAnalyzeReports, RenderError> {
    if let Some(ctx) = try_use_context::<UseAnalyzeReports>() {
        return Ok(ctx);
    }
    let reports = use_loader(move || async move { list_analyze_reports(space_id(), None).await })?;
    Ok(use_context_provider(|| UseAnalyzeReports {
        space_id,
        reports,
    }))
}
