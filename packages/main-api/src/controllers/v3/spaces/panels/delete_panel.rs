use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::features::spaces::panels::SpacePanel;
use crate::models::SpaceCommon;
use crate::models::User;
use crate::types::EntityType;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error};
use bdk::prelude::axum::extract::{Json, Path, State};

pub async fn delete_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
) -> Result<Json<Partition>, Error> {
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

    let panel_id = match panel_pk.clone() {
        Partition::Panel(v) => v.to_string(),
        _ => "".to_string(),
    };

    let panel = SpacePanel::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceDiscussion(panel_id.to_string())),
    )
    .await?;

    if panel.is_none() {
        return Err(Error::NotFoundPanel);
    }

    SpacePanel::delete(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    Ok(Json(panel_pk))
}
