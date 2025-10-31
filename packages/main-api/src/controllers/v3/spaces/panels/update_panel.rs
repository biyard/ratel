use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::features::spaces::panels::SpacePanel;
use crate::features::spaces::panels::SpacePanelRequest;
use crate::features::spaces::panels::SpacePanelResponse;
use crate::models::SpaceCommon;
use crate::models::User;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error};
use bdk::prelude::axum::extract::{Json, Path, State};

pub async fn update_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
    Json(req): Json<SpacePanelRequest>,
) -> Result<Json<SpacePanelResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (pk, sk) = SpacePanel::keys(&space_pk, &panel_pk);

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

    let panel = SpacePanel::updater(&pk, sk)
        .with_name(req.name)
        .with_quotas(req.quotas)
        .with_attributes(req.attributes)
        .execute(&dynamo.client)
        .await?;

    let panel = panel.into();

    Ok(Json(panel))
}
