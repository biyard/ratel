use crate::{
    AppState, Error,
    controllers::v3::spaces::dto::SpacePathParam,
    features::spaces::sprint_leagues::{
        SprintLeagueMetadata, SprintLeagueResponse, SprintLeagueVote,
    },
    models::User,
    types::{EntityType, Partition, TeamGroupPermission},
};
use aide::NoApi;
use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Path, State},
};
use crate::types::Permissions;

pub async fn get_sprint_league_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
) -> crate::Result<Json<SprintLeagueResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::SpaceNotFound);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let sprint_league_future = SprintLeagueMetadata::query_begins_with_sk(
        &dynamo.client,
        &space_pk,
        EntityType::SprintLeague,
    );

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
    Ok(Json((sprint_league, user_vote.is_some()).into()))
}
