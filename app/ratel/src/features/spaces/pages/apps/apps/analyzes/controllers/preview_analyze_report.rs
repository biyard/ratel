use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct PreviewAnalyzeReportRequest {
    pub filters: Vec<AnalyzeReportFilter>,
}

/// Cap on how many sample record refs the preview returns per filter.
/// Lower than the persisted-report cap because preview re-runs on every
/// chip change in the wizard — keeping the response light keeps the UI
/// responsive while still showing enough rows to recognize the data.
pub const PREVIEW_SAMPLE_PER_FILTER: usize = 50;

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
    /// Hydrated sample of matched records grouped per filter (top
    /// `PREVIEW_SAMPLE_PER_FILTER` per chip). Drives the "사용된 raw
    /// data" tables under the count tiles. Empty when filters are
    /// empty. Same `AnalyzeRecordRow` shape as the records page so
    /// the UI's table component is reused verbatim.
    #[serde(default)]
    pub sample_records: Vec<AnalyzeRecordRow>,
}

/// Compute the matching respondent count for a tentative filter set.
/// Used by the CREATE wizard's "Next" step before persisting; the
/// stream-based auto-analysis pipeline reuses the same matching logic
/// (see `services::intersection`) against the saved filters once the
/// report is persisted.
#[post(
    "/api/spaces/{space_id}/apps/analyzes/reports/preview",
    role: SpaceUserRole,
    space: SpaceCommon
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
        // "전체" preview — only the respondent count is meaningful
        // (no chip ⇒ no per-source sample to draw). The frontend
        // hides the data_count tile and the sample section entirely
        // in this branch.
        let participants = services::intersection::count_all_participants(cli, &space_pk).await?;
        return Ok(PreviewAnalyzeReportResponse {
            respondent_count: participants,
            data_count: 0,
            sample_records: Vec::new(),
        });
    }

    let (intersection, data_count, all_records) =
        services::intersection::intersect_filters(cli, &space_pk, &req.filters).await?;

    // Down-sample per-filter so a high-volume chip doesn't dominate the
    // response. The intersection helper already caps each filter at
    // `PER_FILTER_RECORD_CAP` (1000); preview slices that further to
    // 50/filter so the wizard stays snappy.
    let mut per_filter_count: std::collections::HashMap<u32, usize> =
        std::collections::HashMap::new();
    let mut sample_refs: Vec<MatchedRecordRef> = Vec::new();
    for r in all_records {
        let count = per_filter_count.entry(r.filter_idx).or_insert(0);
        if *count < PREVIEW_SAMPLE_PER_FILTER {
            *count += 1;
            sample_refs.push(r);
        }
    }

    let sample_records = services::record_hydrate::hydrate_records(
        cli,
        &space_pk,
        &req.filters,
        sample_refs,
        space.anonymous_participation,
    )
    .await?;

    Ok(PreviewAnalyzeReportResponse {
        respondent_count: intersection.len() as i64,
        data_count,
        sample_records,
    })
}
