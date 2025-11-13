use crate::features::spaces::panels::*;
use crate::*;

// FIXME: request with quota pk or id
pub async fn delete_panel_quota_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(PanelPathParam { space_pk, panel_sk }): PanelPath,
) -> Result<Json<Partition>> {
    permissions.permitted(TeamGroupPermission::SpaceDelete)?;

    let pk = CompositePartition(space_pk.clone(), Partition::PanelAttribute);
    let sk = panel_sk;

    SpacePanelQuota::delete(&dynamo.client, &pk, Some(sk)).await?;

    Ok(Json(space_pk))
}
