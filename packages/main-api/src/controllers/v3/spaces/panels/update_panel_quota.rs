use crate::EntityType;
use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::panels::PanelAttribute;
use crate::features::spaces::panels::SpacePanelQuota;
use crate::features::spaces::panels::SpacePanelRequest;
use crate::features::spaces::panels::SpacePanelsResponse;
use crate::transact_write_items;
use crate::types::Attribute;
use crate::types::CompositePartition;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error, Permissions};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdatePanelQuotaRequest {
    pub quotas: i64,
    pub attribute: PanelAttribute,
    pub value: Attribute,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpdatePanelQuotaResponse {
    pub quotas: i64,
    pub attribute: PanelAttribute,
    pub value: Attribute,
}

pub async fn update_panel_quota_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdatePanelQuotaRequest>,
) -> Result<Json<UpdatePanelQuotaResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let pk = CompositePartition(space_pk, Partition::PanelAttribute);
    let sk = EntityType::SpacePanelAttribute(req.attribute.to_string(), req.value.to_string());

    let panel = SpacePanelQuota::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    if panel.is_none() {
        return Err(Error::NotFoundPanel);
    }

    let panel = panel.unwrap_or_default();
    let remains = panel.remains + (req.quotas - panel.quotas);

    let _ = SpacePanelQuota::updater(&pk, sk)
        .with_quotas(req.quotas)
        .with_remains(remains)
        .execute(&dynamo.client)
        .await?;

    Ok(Json(UpdatePanelQuotaResponse {
        quotas: req.quotas.clone(),
        attribute: req.attribute,
        value: req.value,
    }))
}
