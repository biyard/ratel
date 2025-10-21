use crate::{
    AppState, Error,
    controllers::v3::spaces::dto::SpacePathParam,
    features::spaces::sprint_leagues::{
        SprintLeagueMetadata, SprintLeagueResponse, SprintLeagueVote,
    },
    models::{SpaceCommon, User},
    types::{EntityType, Partition, TeamGroupPermission},
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
