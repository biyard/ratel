use crate::features::spaces::polls::*;
use crate::{
    AppState, Error2,
    models::{space::SpaceCommon, user::User},
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
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
) -> Result<Json<PollResponse>, Error2> {
    // Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundPoll);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let metadata = PollMetadata::query_begins_with_sk(&dynamo.client, &space_pk, &poll_sk).await?;
    let mut poll_response: PollResponse = PollResponse::from(metadata);
    if let Some(user) = user {
        let my_survey_response =
            PollUserAnswer::find_one(&dynamo.client, &space_pk, &user.pk).await?;

        poll_response.my_response = my_survey_response.map(|r| r.answers);
    }

    Ok(Json(poll_response))
}
