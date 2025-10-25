use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::dto::space_discussion_response::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::{AppState, Error, models::user::User};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn exit_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<SpaceDiscussionResponse>, Error> {
    let (pk, sk) = SpaceDiscussion::keys(&space_pk, &discussion_pk);
    let disc = SpaceDiscussion::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    if disc.is_none() {
        return Err(Error::NotFoundDiscussion);
    }

    let disc = disc.unwrap();
    if disc.meeting_id.is_none() {
        return Err(Error::AwsChimeError("Not Found Meeting ID".into()));
    }

    let (p_pk, p_sk) = SpaceDiscussionParticipant::keys(&discussion_pk, &user.pk);

    let participant =
        SpaceDiscussionParticipant::get(&dynamo.client, p_pk.clone(), Some(p_sk.clone())).await?;

    if participant.is_none() {
        return Err(Error::AwsChimeError("Not Found Participant".into()));
    }

    let _ = SpaceDiscussionParticipant::delete(&dynamo.client, p_pk, Some(p_sk)).await?;

    let discussion =
        SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
            .await?;

    Ok(Json(discussion))
}
