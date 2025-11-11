use crate::Attribute;
use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::controllers::v3::spaces::SpacePath;
use crate::features::spaces::panels::PanelAttribute;
use crate::features::spaces::panels::SpacePanelQuota;
use crate::spaces::SpacePathParam;
use crate::types::CompositePartition;
use crate::types::EntityType;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error, Permissions};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct DeletePanelQuotaRequest {
    pub attribute: PanelAttribute,
    pub value: Attribute,
}

pub async fn delete_panel_quota_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<DeletePanelQuotaRequest>,
) -> Result<Json<Partition>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceDelete) {
        return Err(Error::NoPermission);
    }

    let pk = CompositePartition(space_pk.clone(), Partition::PanelAttribute);
    let sk = EntityType::SpacePanelAttribute(req.attribute.to_string(), req.value.to_string());

    let panel = SpacePanelQuota::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    if panel.is_none() {
        return Err(Error::NotFoundPanel);
    }

    SpacePanelQuota::delete(&dynamo.client, &pk, Some(sk)).await?;

    Ok(Json(space_pk))
}
