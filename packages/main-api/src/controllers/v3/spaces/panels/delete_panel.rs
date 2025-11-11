use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::features::spaces::panels::SpacePanel;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error, Permissions};
use bdk::prelude::axum::extract::{Json, Path, State};

pub async fn delete_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
) -> Result<Json<Partition>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceDelete) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpacePanel::keys(&space_pk, &panel_pk);

    let panel = SpacePanel::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    if panel.is_none() {
        return Err(Error::NotFoundPanel);
    }

    SpacePanel::delete(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    Ok(Json(panel_pk))
}
