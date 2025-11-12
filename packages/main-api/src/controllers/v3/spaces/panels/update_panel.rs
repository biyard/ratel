use crate::features::spaces::panels::PanelAttribute;
use crate::features::spaces::panels::SpacePanels;
use crate::features::spaces::panels::SpacePanelsResponse;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpdatePanelRequest {
    pub quotas: i64,
    pub attributes: Vec<PanelAttribute>,
}

pub async fn update_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdatePanelRequest>,
) -> Result<Json<SpacePanelsResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let panel = SpacePanels::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpacePanels),
    )
    .await?
    .unwrap_or_default();

    let remains = panel.remains + (req.quotas - panel.quotas);

    let panel = SpacePanels::updater(&space_pk, EntityType::SpacePanels)
        .with_quotas(req.quotas)
        .with_remains(remains)
        .with_attributes(req.attributes)
        .execute(&dynamo.client)
        .await?;

    let panel = panel.into();

    Ok(Json(panel))
}
