use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::features::spaces::panels::SpacePanel;
use crate::features::spaces::panels::SpacePanelRequest;
use crate::features::spaces::panels::SpacePanelResponse;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error, Permissions};
use bdk::prelude::axum::extract::{Json, Path, State};

pub async fn update_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
    Json(req): Json<SpacePanelRequest>,
) -> Result<Json<SpacePanelResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpacePanel::keys(&space_pk, &panel_pk);

    let panel = SpacePanel::updater(&pk, sk)
        .with_name(req.name)
        .with_quotas(req.quotas)
        .with_attributes(req.attributes)
        .execute(&dynamo.client)
        .await?;

    let panel = panel.into();

    Ok(Json(panel))
}
