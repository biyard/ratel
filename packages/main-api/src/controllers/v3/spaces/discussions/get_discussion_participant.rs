use crate::controllers::v3::spaces::SpaceDiscussionPath;
use crate::controllers::v3::spaces::SpaceDiscussionPathParam;
use crate::features::spaces::discussions::dto::ListDiscussionParticipantResponse;
use crate::features::spaces::discussions::dto::ListDiscussionQueryParams;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipantQueryOption;
use crate::types::Partition;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;

pub async fn get_discussion_participants_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
    Query(ListDiscussionQueryParams { bookmark }): Query<ListDiscussionQueryParams>,
) -> Result<Json<ListDiscussionParticipantResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundSpace);
    }

    let mut query_options = SpaceDiscussionParticipantQueryOption::builder()
        .sk("SPACE_DISCUSSION_PARTICIPANT#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (participants, bookmark) =
        SpaceDiscussionParticipant::query(&dynamo.client, discussion_pk.clone(), query_options)
            .await?;

    Ok(Json(ListDiscussionParticipantResponse {
        participants,
        bookmark,
    }))
}
