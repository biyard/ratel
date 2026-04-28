//! Returns the list of user pks (`USER#...`) that match a report's
//! cross-filter selection. Used by the Excel download path so the
//! exported workbook contains only filtered respondents instead of
//! the entire space.
//!
//! Empty filter list returns every space participant — same fallback
//! semantics as the preview / auto-analysis services.

use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[get(
    "/api/spaces/{space_id}/apps/analyzes/reports/{report_id}/matched_users",
    role: SpaceUserRole
)]
pub async fn get_matched_users(
    space_id: SpacePartition,
    report_id: SpaceAnalyzeReportEntityType,
) -> Result<Vec<String>> {
    SpaceApp::can_view(role)?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let report_sk: EntityType = report_id.into();
    let report = match SpaceAnalyzeReport::get(cli, &space_pk, Some(report_sk)).await {
        Ok(Some(r)) => r,
        Ok(None) => return Ok(Vec::new()),
        Err(e) => {
            crate::error!("get_matched_users get report: {e}");
            return Err(Error::Internal);
        }
    };

    let pks = if report.filters.is_empty() {
        services::intersection::list_participant_user_pks(cli, &space_pk)
            .await
            .map_err(|e| {
                crate::error!("get_matched_users participants: {e}");
                Error::Internal
            })?
            .into_iter()
            .map(|p| p.to_string())
            .collect()
    } else {
        let (set, _) =
            services::intersection::intersect_filters(cli, &space_pk, &report.filters)
                .await
                .map_err(|e| {
                    crate::error!("get_matched_users intersect: {e}");
                    Error::Internal
                })?;
        set.into_iter().collect()
    };

    Ok(pks)
}
