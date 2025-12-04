use crate::features::spaces::reports::dto::PublishReportResponse;
use crate::features::spaces::SpaceReport;
use crate::spaces::{SpacePath, SpacePathParam};
use crate::types::{EntityType, Partition, ReportPublishState, SpacePartition, TeamGroupPermission};
use crate::utils::time::get_now_timestamp_millis;
use crate::*;

/// Publishes a report for sale.
/// Prerequisites:
/// - Report must exist
/// - Pricing must be set (address verified)
/// - Report cannot already be published
pub async fn publish_report_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> crate::Result<Json<PublishReportResponse>> {
    // Validate space_pk
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    // Check permissions - only space admin can publish
    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    // Get existing report
    let mut report =
        SpaceReport::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceReport))
            .await?
            .ok_or(Error::NotFound("Report not found".to_string()))?;

    // Check if already published
    if report.publish_state == ReportPublishState::Published {
        return Err(Error::BadRequest("Report is already published".to_string()));
    }

    // Check if pricing is set
    if !report.can_publish() {
        return Err(Error::BadRequest(
            "Cannot publish report. Ensure pricing is set and recipient address is verified."
                .to_string(),
        ));
    }

    // Get price for response
    let price_dollars = report
        .price_dollars
        .ok_or(Error::BadRequest("Price not set".to_string()))?;

    // Publish the report
    let now = get_now_timestamp_millis();
    report.publish_state = ReportPublishState::Published;
    report.published_at = Some(now);
    report.gsi1_pk = Some("REPORT_PUBLISHED".to_string());
    report.gsi1_sk = Some(now.to_string());
    report.updated_at = now;

    report.upsert(&dynamo.client).await?;

    let space_id: SpacePartition = space_pk.into();

    // Generate x402 resource URL
    // This will be the endpoint where paid content can be accessed
    let x402_resource = format!("/v3/spaces/{}/reports/content", space_id.to_string());

    Ok(Json(PublishReportResponse {
        space_id,
        published_at: now,
        price_dollars,
        x402_resource,
    }))
}
