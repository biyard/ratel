use crate::controllers::v3::spaces::{SpaceDiscussionPath, SpaceDiscussionPathParam};
use crate::features::spaces::discussions::dto::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::{AppState, Error2, models::user::User};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;

use aide::NoApi;

pub async fn start_meeting_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpaceDiscussionPathParam {
        space_pk,
        discussion_pk,
    }): SpaceDiscussionPath,
) -> Result<Json<SpaceDiscussionResponse>, Error2> {
    let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
    let (pk, sk) = SpaceDiscussion::keys(&space_pk, &discussion_pk);
    let disc = SpaceDiscussion::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    if disc.is_none() {
        return Err(Error2::NotFoundDiscussion);
    }

    let disc = disc.unwrap();

    let _ = client
        .ensure_current_meeting(
            dynamo.clone(),
            &client,
            space_pk.clone(),
            discussion_pk.clone(),
            &disc,
        )
        .await;

    let discussion =
        SpaceDiscussion::get_discussion(&dynamo.client, &space_pk, &discussion_pk, &user.pk)
            .await?;

    Ok(Json(discussion))
}
