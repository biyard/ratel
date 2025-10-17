// use aide::NoApi;
// use bdk::prelude::*;
// use by_axum::axum::{
//     Json,
//     extract::{Path, Query, State},
// };

// use crate::{AppState, models::User};

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
// pub struct GetSprintLeaguePathParams {}

// #[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
// pub struct GetSprintLeagueResponse {}

// #[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
// pub struct GetSprintLeagueQueryParams {}

// pub async fn get_sprint_league_handler(
//     State(AppState { dynamo, .. }): State<AppState>,
//     NoApi(user): NoApi<Option<User>>,
//     Path(path): Path<GetSprintLeaguePathParams>,
//     Query(query): Query<GetSprintLeagueQueryParams>,
// ) -> crate::Result<Json<GetSprintLeagueResponse>> {
//     Ok(Json(GetSprintLeagueResponse::default()))
// }

use super::dto::SprintLeagueResponse;
use crate::{
    AppState, Error,
    controllers::v3::spaces::dto::SpacePathParam,
    models::{SpaceCommon, SprintLeagueMetadata, SprintLeagueVote, User},
    types::{Partition, TeamGroupPermission},
};
use aide::NoApi;
use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Path, State},
};

pub async fn get_sprint_league_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
) -> crate::Result<Json<SprintLeagueResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::SpaceNotFound);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let sprint_league_future = SprintLeagueMetadata::query(&dynamo.client, &space_pk);
    let user_vote_future = if let Some(user) = &user {
        Some(SprintLeagueVote::find_one(
            &dynamo.client,
            &space_pk,
            &user.pk,
        ))
    } else {
        None
    };

    let (sprint_league, user_vote) = if let Some(vote_future) = user_vote_future {
        tokio::try_join!(sprint_league_future, vote_future)?
    } else {
        let sprint_league = sprint_league_future.await?;
        (sprint_league, None)
    };

    let mut response: SprintLeagueResponse = sprint_league.into();
    if user_vote.is_some() {
        response.is_voted = true;
    }

    Ok(Json(response))
}
