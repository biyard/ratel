//! Hydrate the report's frozen `MatchedRecordRef` list back into
//! displayable rows for the "사용된 데이터 확인하기" detail page.
//!
//! One filter chip at a time (caller passes `filter_idx`) so the page
//! UI can drive a tab/accordion per chip without scanning the whole
//! report. Pagination is in-memory offset over the report's stored
//! `matched_records` (capped per filter at 1000 by intersection.rs),
//! so we never hit DDB to walk the snapshot itself — only to hydrate
//! the per-row display fields.

use crate::common::ListResponse;
use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

/// Page size for the records view. Picked so a typical filter
/// (≤1000 matches) tops out at 20 pages — comfortable to scroll
/// without producing per-request payloads big enough to slow the wire.
const PAGE_SIZE: usize = 50;

/// Paginated, hydrated list of frozen records belonging to one
/// filter chip on a saved report. `filter_idx` is required so the
/// page can render a tab per chip without paying the hydration cost
/// for chips the user isn't looking at.
#[get(
    "/api/spaces/{space_id}/apps/analyzes/reports/{report_id}/records?filter_idx&bookmark",
    role: SpaceUserRole,
    space: SpaceCommon
)]
pub async fn list_analyze_records(
    space_id: SpacePartition,
    report_id: SpaceAnalyzeReportEntityType,
    filter_idx: Option<u32>,
    bookmark: Option<String>,
) -> Result<ListResponse<AnalyzeRecordRow>> {
    SpaceApp::can_view(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let sk: EntityType = report_id.into();

    let report = SpaceAnalyzeReport::get(cli, space_pk.clone(), Some(sk))
        .await
        .map_err(|e| {
            crate::error!("list_analyze_records: failed to load report: {e}");
            Error::Internal
        })?
        .ok_or(Error::NotFound("analyze report".to_string()))?;

    let filter_idx = match filter_idx {
        Some(v) => v,
        None => return Ok(ListResponse::default()),
    };

    // Filter the report's frozen ref list down to the requested chip,
    // then offset/limit in-memory. Bookmark is the next-page offset
    // as a decimal string — there's no underlying DDB pagination to
    // round-trip, so we keep it human-readable.
    let filtered: Vec<&MatchedRecordRef> = report
        .matched_records
        .iter()
        .filter(|r| r.filter_idx == filter_idx)
        .collect();

    let offset: usize = bookmark
        .as_deref()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let end = (offset + PAGE_SIZE).min(filtered.len());
    let page: Vec<MatchedRecordRef> = filtered[offset..end].iter().map(|r| (*r).clone()).collect();
    let next_bookmark = if end < filtered.len() {
        Some(end.to_string())
    } else {
        None
    };

    let items = services::record_hydrate::hydrate_records(
        cli,
        &space_pk,
        &report.filters,
        page,
        space.anonymous_participation,
    )
    .await?;

    Ok(ListResponse {
        items,
        bookmark: next_bookmark,
    })
}
