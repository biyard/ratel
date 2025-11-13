use crate::features::spaces::panels::*;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdatePanelQuotaRequest {
    pub quota: i64,
}

pub async fn update_panel_quota_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(PanelPathParam { space_pk, panel_sk }): PanelPath,
    Json(req): Json<UpdatePanelQuotaRequest>,
) -> Result<Json<SpacePanelQuota>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let pk = CompositePartition(space_pk, Partition::PanelAttribute);
    let sk = panel_sk;

    // NOTE: we don't consider to update after publishing.
    let space_panel = SpacePanelQuota::updater(&pk, sk)
        .with_quotas(req.quota)
        .execute(&dynamo.client)
        .await?;

    Ok(Json(space_panel))
}
