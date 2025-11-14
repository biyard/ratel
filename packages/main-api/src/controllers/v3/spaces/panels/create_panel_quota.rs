use crate::features::spaces::panels::PanelAttribute;
use crate::features::spaces::panels::PanelAttributeWithQuota;
use crate::features::spaces::panels::SpacePanelQuota;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct CreatePanelQuotaRequest {
    pub attributes: Vec<PanelAttributeWithQuota>,
}

pub async fn create_panel_quota_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreatePanelQuotaRequest>,
) -> Result<Json<Vec<SpacePanelQuota>>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let panels: Vec<SpacePanelQuota> = req
        .attributes
        .into_iter()
        .map(|e| {
            let panel: SpacePanelQuota = (space_pk.clone(), e).into();

            panel
        })
        .collect();

    let tx = panels
        .clone()
        .into_iter()
        .map(|e| e.create_transact_write_item())
        .collect();

    transact_write_items!(dynamo.client, tx)?;

    Ok(Json(panels))
}
