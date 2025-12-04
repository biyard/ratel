use crate::features::spaces::reports::dto::{CreateReportRequest, CreateReportResponse};
use crate::features::spaces::SpaceReport;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::spaces::{SpacePath, SpacePathParam};
use crate::types::{EntityType, Partition, SpacePartition, TeamGroupPermission};
use crate::*;

pub async fn create_report_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Extension(_space): Extension<SpaceCommon>,
    Json(req): Json<CreateReportRequest>,
) -> crate::Result<Json<CreateReportResponse>> {
    // Validate space_pk
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    // Check permissions - only space admin can create report
    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    // Check if report already exists
    let existing_report =
        SpaceReport::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceReport)).await?;

    if existing_report.is_some() {
        return Err(Error::AlreadyExists("Report already exists for this space".to_string()));
    }

    // Create new report
    let report = SpaceReport::new(
        space_pk.clone(),
        user.pk.clone(),
        user.display_name.clone(),
        user.username.clone(),
    )
    .set_report_content(req.title.clone(), req.content, req.summary);

    report.create(&dynamo.client).await?;

    let space_id: SpacePartition = space_pk.into();

    Ok(Json(CreateReportResponse {
        space_id,
        title: req.title,
        publish_state: report.publish_state,
        created_at: report.created_at,
    }))
}
