use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::features::spaces::panels::SpacePanel;
use crate::features::spaces::panels::SpacePanelResponse;
use crate::types::Partition;
use crate::{AppState, Error, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
) -> Result<Json<SpacePanelResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (pk, sk) = SpacePanel::keys(&space_pk, &panel_pk);

    let panel = SpacePanel::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    let panel = panel.unwrap_or_default();
    let panel = panel.into();
    Ok(Json(panel))
}
