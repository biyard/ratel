use crate::features::spaces::panels::PanelAttribute;
use crate::features::spaces::panels::SpacePanelQuota;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct CreatePanelQuotaRequest {
    pub quotas: Vec<i64>,
    pub attributes: Vec<PanelAttribute>,
    pub values: Vec<Attribute>,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct CreatePanelQuotaResponse {
    pub quotas: Vec<i64>,
    pub attributes: Vec<PanelAttribute>,
    pub values: Vec<Attribute>,
}

pub async fn create_panel_quota_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreatePanelQuotaRequest>,
) -> Result<Json<CreatePanelQuotaResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions
        .permitted(TeamGroupPermission::SpaceEdit)
        .require()?;

    let mut tx = vec![];

    for (i, _) in req.attributes.iter().enumerate() {
        let quota = req.quotas[i];
        let attribute = req.attributes[i];
        let value = req.values[i].clone();

        let panel = SpacePanelQuota::new(
            space_pk.clone(),
            attribute.to_string(),
            value.to_string(),
            quota,
            attribute.clone(),
        );

        tx.push(panel.create_transact_write_item());
    }

    transact_write_items!(dynamo.client, tx)?;

    Ok(Json(CreatePanelQuotaResponse {
        quotas: req.quotas.clone(),
        attributes: req.attributes.clone(),
        values: req.values.clone(),
    }))
}
