use crate::features::spaces::panels::{PanelAttribute, SpacePanels, SpacePanelsResponse};
use crate::spaces::{SpacePath, SpacePathParam};
use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpdateSpacePanelsRequest {
    pub quotas: i64,
    pub attributes: Vec<PanelAttribute>,
}

pub async fn update_space_panels_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateSpacePanelsRequest>,
) -> Result<Json<SpacePanelsResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let panels = SpacePanels::new(space_pk.clone(), req.quotas, req.attributes);

    panels.upsert(&dynamo.client).await?;

    let response = SpacePanelsResponse {
        quotas: panels.quotas,
        remains: panels.remains,
    };

    Ok(Json(response))
}
