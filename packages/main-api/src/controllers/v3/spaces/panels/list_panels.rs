use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::features::spaces::panels::ListPanelQueryParams;
use crate::features::spaces::panels::ListPanelResponse;
use crate::features::spaces::panels::SpacePanel;
use crate::features::spaces::panels::SpacePanelQueryOption;
use crate::features::spaces::panels::SpacePanelResponse;
use crate::models::User;
use crate::types::Partition;
use crate::{AppState, Error};
use bdk::prelude::axum::extract::{Json, Path, Query, State};

pub async fn list_panels_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
    Query(ListPanelQueryParams { bookmark }): Query<ListPanelQueryParams>,
) -> Result<Json<ListPanelResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let mut query_options = SpacePanelQueryOption::builder()
        .sk(format!("SPACE_PANEL#{}", panel_pk))
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (responses, bookmark) =
        SpacePanel::query(&dynamo.client, space_pk.clone(), query_options).await?;

    let mut panels = vec![];

    for response in responses {
        let panel: SpacePanelResponse = response.clone().into();
        panels.push(panel);
    }

    Ok(Json(ListPanelResponse { panels, bookmark }))
}
