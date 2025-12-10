use crate::features::spaces::SpaceReport;
use crate::features::spaces::reports::dto::{CreateReportRequest, CreateReportResponse};
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::spaces::{SpacePath, SpacePathParam};
use crate::types::{EntityType, Partition, SpacePartition, TeamGroupPermission};
use crate::*;

pub async fn create_report_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Extension(space): Extension<SpaceCommon>,
    Json(req): Json<CreateReportRequest>,
) -> crate::Result<Json<CreateReportResponse>> {
    // Check permissions - only space admin can create report
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let space_pk = space.pk.clone();

    // Create new report
    let mut report = SpaceReport::new(
        space_pk.clone(),
        user.pk.clone(),
        user.display_name.clone(),
        user.username.clone(),
    )
    .with_title(req.title.clone())
    .with_content(req.content);

    if let Some(summary) = req.summary {
        report = report.with_summary(summary);
    }

    report.create(&dynamo.client).await.map_err(|e| {
        error!("Failed to create report: {:?}", e);

        Error::AlreadyExists("Report may already exists for this space".to_string())
    })?;

    let space_id: SpacePartition = space_pk.into();

    Ok(Json(CreateReportResponse {
        space_id,
        title: req.title,
        publish_state: report.publish_state,
        created_at: report.created_at,
    }))
}
