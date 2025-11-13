use crate::{features::spaces::panels::SpacePanelQuota, models::SpaceCommon};

use super::*;

pub async fn list_panel_attributes_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(space): Extension<SpaceCommon>,
    Query(Pagination { bookmark }): ListItemsQuery,
) -> Result<Json<ListItemsResponse<SpacePanelQuota>>> {
    let opt = SpacePanelQuota::opt_with_bookmark(bookmark);

    let res = SpacePanelQuota::query(
        &dynamo.client,
        CompositePartition(space.pk, Partition::PanelAttribute),
        opt,
    )
    .await?;

    Ok(Json(res.into()))
}
