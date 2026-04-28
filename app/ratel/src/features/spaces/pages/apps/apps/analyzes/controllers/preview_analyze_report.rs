use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct PreviewAnalyzeReportRequest {
    pub filters: Vec<AnalyzeReportFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct PreviewAnalyzeReportResponse {
    /// Distinct user_pk count matching every chip (AND across all
    /// chips). When the filter list is empty, this is the count of all
    /// space participants — i.e. the unrestricted denominator.
    pub respondent_count: i64,
    /// Sum of underlying records across the chosen sources. Mirrors the
    /// "해당되는 데이터 수" tile in the preview card.
    pub data_count: i64,
}

/// Compute the matching respondent count for a tentative filter set.
/// Used by the CREATE wizard's "Next" step before persisting; the
/// stream-based auto-analysis pipeline reuses the same matching logic
/// (see `services::intersection`) against the saved filters once the
/// report is persisted.
#[post(
    "/api/spaces/{space_id}/apps/analyzes/reports/preview",
    role: SpaceUserRole
)]
pub async fn preview_analyze_report(
    space_id: SpacePartition,
    req: PreviewAnalyzeReportRequest,
) -> Result<PreviewAnalyzeReportResponse> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    if req.filters.is_empty() {
        let participants = services::intersection::count_all_participants(cli, &space_pk).await?;
        return Ok(PreviewAnalyzeReportResponse {
            respondent_count: participants,
            data_count: 0,
        });
    }

    let (intersection, data_count) =
        services::intersection::intersect_filters(cli, &space_pk, &req.filters).await?;

    Ok(PreviewAnalyzeReportResponse {
        respondent_count: intersection.len() as i64,
        data_count,
    })
}
