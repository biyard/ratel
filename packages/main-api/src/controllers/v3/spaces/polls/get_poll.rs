use crate::features::spaces::polls::*;
use crate::models::space::SpaceCommon;
use crate::types::{EntityType, SpacePublishState};
use crate::{
    AppState, Error, Permissions,
    models::user::User,
    types::{Partition, TeamGroupPermission},
};

use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Path, State},
};

use aide::NoApi;

pub async fn get_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    NoApi(permissions): NoApi<Permissions>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
) -> Result<Json<PollResponse>, Error> {
    // Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundPoll);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let sc = SpaceCommon::get(&dynamo.client, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::NotFoundSpace)?;

    let poll_pk = match poll_sk.clone() {
        EntityType::SpacePoll(v) => Partition::Poll(v.to_string()),
        _ => Partition::Poll("".to_string()),
    };

    let poll = Poll::get(&dynamo.client, &space_pk, Some(&poll_sk))
        .await?
        .ok_or(Error::NotFoundPoll)?;

    let mut poll_response: PollResponse = PollResponse::from(poll);
    let now = crate::utils::time::get_now_timestamp_millis();

    if user.is_some()
        && sc.publish_state == SpacePublishState::Published
        && poll_response.started_at <= now
    {
        let user = user.unwrap();
        let my_survey_response =
            PollUserAnswer::find_one(&dynamo.client, &space_pk, &poll_pk, &user.pk).await?;

        poll_response.my_response = my_survey_response.map(|r| r.answers);
    }

    Ok(Json(poll_response))
}
