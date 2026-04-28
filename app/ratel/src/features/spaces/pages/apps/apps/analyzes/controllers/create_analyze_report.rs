use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct CreateAnalyzeReportRequest {
    pub name: String,
    pub filters: Vec<AnalyzeReportFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct CreateAnalyzeReportResponse {
    pub report_id: String,
}

/// Persist a new analyze report with `status = InProgress`. The actual
/// LDA / TF-IDF / poll-quiz aggregation pipeline (next stage) reads the
/// stored `filters` off this row, writes its result fields back onto
/// the same row, and flips `status` to `Finish`. The response carries
/// just the new id so the client can navigate to the detail view (or
/// stay on the list and watch the badge update).
#[post("/api/spaces/{space_id}/apps/analyzes/reports", role: SpaceUserRole)]
pub async fn create_analyze_report(
    space_id: SpacePartition,
    req: CreateAnalyzeReportRequest,
) -> Result<CreateAnalyzeReportResponse> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let trimmed_name = req.name.trim();
    if trimmed_name.is_empty() {
        return Err(Error::InvalidFormat);
    }

    let report = SpaceAnalyzeReport::new(space_id, trimmed_name.to_string(), req.filters);
    let report_id = match &report.sk {
        EntityType::SpaceAnalyzeReport(id) => id.clone(),
        _ => String::new(),
    };

    report.create(cli).await.map_err(|e| {
        crate::error!("failed to create analyze report: {e}");
        Error::Internal
    })?;

    Ok(CreateAnalyzeReportResponse { report_id })
}
