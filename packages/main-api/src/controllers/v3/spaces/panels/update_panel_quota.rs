use crate::features::spaces::panels::PanelAttribute;
use crate::features::spaces::panels::SpacePanelQuota;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

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
) -> Result<Json<UpdatePanelQuotaResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let pk = CompositePartition(space_pk, Partition::PanelAttribute);
    let sk = EntityType::SpacePanelAttribute(req.attribute.to_key(), req.value.to_string());

    let panel = SpacePanelQuota::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    if panel.is_none() {
        return Err(Error::NotFoundPanel);
    }

    // TODO: update verifiable attribute on space panels.

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
