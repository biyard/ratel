use crate::features::spaces::reports::dto::UpdateReportRequest;
use crate::features::spaces::SpaceReport;
use crate::spaces::{SpacePath, SpacePathParam};
use crate::types::{EntityType, Partition, ReportPublishState, TeamGroupPermission};
use crate::utils::time::get_now_timestamp_millis;
use crate::*;

pub async fn update_report_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateReportRequest>,
) -> crate::Result<Json<serde_json::Value>> {
    // Validate space_pk
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    // Check permissions - only space admin can update report
    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    // Get existing report
    let mut report =
        SpaceReport::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceReport))
            .await?
            .ok_or(Error::NotFound("Report not found".to_string()))?;

    // Cannot update published reports
    if report.publish_state == ReportPublishState::Published {
        return Err(Error::BadRequest(
            "Cannot update published report".to_string(),
        ));
    }

    // Update fields if provided
    if let Some(title) = req.title {
        report.title = title;
    }
    if let Some(content) = req.content {
        report.content = content;
    }
    if req.summary.is_some() {
        report.summary = req.summary;
    }

    report.updated_at = get_now_timestamp_millis();

    // Update in DynamoDB
    report.upsert(&dynamo.client).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "updated_at": report.updated_at
    })))
}
