use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::polls::*;
use crate::types::SpacePublishState;
use crate::{
    AppState, Error,
    models::{space::SpaceCommon, user::User},
    types::{Partition, TeamGroupPermission},
};

use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Path, Query, State},
};

use aide::NoApi;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListPollQueryParams {
    pub bookmark: Option<String>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ListPollsResponse {
    pub polls: Vec<PollResponse>,
    pub bookmark: Option<String>,
}

pub async fn list_polls_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListPollQueryParams { bookmark }): Query<ListPollQueryParams>,
) -> Result<Json<ListPollsResponse>, Error> {
    // Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundPoll);
    }

    let (_sc, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let mut query_options = PollQueryOption::builder()
        .sk("SPACE_POLL#".into())
        .limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (responses, bookmark) =
        Poll::query(&dynamo.client, space_pk.clone(), query_options).await?;

    let mut polls = vec![];

    for response in responses {
        let poll: PollResponse = response.clone().into();

        polls.push(poll);
    }

    Ok(Json(ListPollsResponse { polls, bookmark }))
}
