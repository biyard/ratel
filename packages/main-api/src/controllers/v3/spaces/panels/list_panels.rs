use crate::aide::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
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
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListPanelQueryParams { bookmark }): Query<ListPanelQueryParams>,
) -> Result<Json<ListPanelResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let mut query_options = SpacePanelQueryOption::builder()
        .sk("SPACE_PANEL#".into())
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
