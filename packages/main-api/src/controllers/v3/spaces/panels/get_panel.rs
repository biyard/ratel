use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::panels::SpacePanelQuota;
use crate::features::spaces::panels::SpacePanels;
use crate::features::spaces::panels::SpacePanelsResponse;
use crate::types::CompositePartition;
use crate::types::EntityType;
use crate::types::Partition;
use crate::{AppState, Error, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

pub async fn get_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<SpacePanelsResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let pk = space_pk.clone();
    let sk = EntityType::SpacePanels;

    let panel = SpacePanels::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    let panel = panel.unwrap_or_default();
    let mut panel: SpacePanelsResponse = panel.into();

    let panel_quota = SpacePanelQuota::query(
        &dynamo.client,
        CompositePartition(pk, Partition::PanelAttribute),
        SpacePanelQuota::opt_all().sk("SPACE_PANEL_ATTRIBUTE#".to_string()),
    )
    .await?
    .0;

    panel.panel_quotas = panel_quota;
    Ok(Json(panel))
}
