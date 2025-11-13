use crate::features::spaces::panels::ListPanelQueryParams;
use crate::features::spaces::panels::ListParticipantResponse;
use crate::features::spaces::panels::SpacePanelParticipant;
use crate::features::spaces::panels::SpacePanelParticipantQueryOption;
use crate::features::spaces::panels::SpacePanelParticipantResponse;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

pub async fn list_participants_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListPanelQueryParams { bookmark }): Query<ListPanelQueryParams>,
) -> Result<Json<ListParticipantResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let mut query_options = SpacePanelParticipantQueryOption::builder()
        .sk("SPACE_PANEL_PARTICIPANT#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (responses, bookmark) =
        SpacePanelParticipant::query(&dynamo.client, space_pk.clone(), query_options).await?;

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
