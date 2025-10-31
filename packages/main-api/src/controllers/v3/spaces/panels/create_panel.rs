use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::panels::SpacePanel;
use crate::features::spaces::panels::SpacePanelRequest;
use crate::features::spaces::panels::SpacePanelResponse;
use crate::models::SpaceCommon;
use crate::models::User;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error};
use bdk::prelude::axum::extract::{Json, Path, State};

pub async fn create_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<SpacePanelRequest>,
) -> Result<Json<SpacePanelResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let panel = SpacePanel::new(space_pk.clone(), req.name, req.quotas, req.attributes);
    panel.create(&dynamo.client).await?;

    let panel = panel.into();

    Ok(Json(panel))
}
