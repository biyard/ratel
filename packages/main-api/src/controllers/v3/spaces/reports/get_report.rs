use crate::features::spaces::reports::dto::{GetReportResponse, RevenueSplitInfo};
use crate::features::spaces::SpaceReport;
use crate::spaces::{SpacePath, SpacePathParam};
use crate::types::{EntityType, Partition, SpacePartition, TeamGroupPermission};
use crate::*;

pub async fn get_report_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> crate::Result<Json<GetReportResponse>> {
    // Validate space_pk
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    // Check permissions - only space members can read report
    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    // Get report
    let report = SpaceReport::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceReport))
        .await?
        .ok_or(Error::NotFound("Report not found".to_string()))?;

    let space_id: SpacePartition = space_pk.into();

    let revenue_split = report.price_dollars.map(|price| {
        RevenueSplitInfo::new(
            price,
            report.treasury_percent,
            report.platform_percent,
            report.creator_percent,
        )
    });

    Ok(Json(GetReportResponse {
        space_id,
        title: report.title,
        content: report.content,
        summary: report.summary,
        price_dollars: report.price_dollars,
        recipient_address: report.recipient_address,
        publish_state: report.publish_state,
        published_at: report.published_at,
        revenue_split,
        author_display_name: report.author_display_name,
        author_username: report.author_username,
        created_at: report.created_at,
        updated_at: report.updated_at,
    }))
}
