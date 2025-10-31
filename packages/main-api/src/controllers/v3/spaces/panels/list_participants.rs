use crate::aide::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::panels::SpacePanelResponse;
use crate::features::spaces::panels::{ListPanelQueryParams, ListParticipantResponse};
use crate::features::spaces::panels::{ListPanelResponse, SpacePanelParticipant};
use crate::features::spaces::panels::{SpacePanel, SpacePanelParticipantQueryOption};
use crate::features::spaces::panels::{SpacePanelParticipantResponse, SpacePanelQueryOption};
use crate::models::User;
use crate::spaces::SpacePanelPath;
use crate::spaces::SpacePanelPathParam;
use crate::types::Partition;
use crate::{AppState, Error};
use bdk::prelude::axum::extract::{Json, Path, Query, State};

pub async fn list_participants_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
    Query(ListPanelQueryParams { bookmark }): Query<ListPanelQueryParams>,
) -> Result<Json<ListParticipantResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let mut query_options = SpacePanelParticipantQueryOption::builder()
        .sk("SPACE_PANEL_PARTICIPANT#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (responses, bookmark) =
        SpacePanelParticipant::query(&dynamo.client, panel_pk.clone(), query_options).await?;

    let mut participants = vec![];

    for response in responses {
        let participant: SpacePanelParticipantResponse = response.clone().into();
        participants.push(participant);
    }

    Ok(Json(ListParticipantResponse {
        participants,
        bookmark,
    }))
}
