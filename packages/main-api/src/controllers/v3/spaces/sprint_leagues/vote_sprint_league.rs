use crate::{
    AppState,
    controllers::v3::spaces::dto::SpacePathParam,
    error::Error,
    models::{SpaceCommon, SprintLeague, SprintLeaguePlayer, SprintLeagueVote, User},
    types::{EntityType, TeamGroupPermission},
};
use aide::{NoApi, OperationIo};
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, OperationIo, JsonSchema)]
pub struct VoteSprintLeagueRequest {
    player_sk: EntityType,
    referral_code: Option<String>,
}

#[derive(Debug, Serialize, Default, OperationIo, JsonSchema)]
pub struct VoteSprintLeagueResponse {}

pub async fn vote_sprint_league_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<VoteSprintLeagueRequest>,
) -> crate::Result<Json<VoteSprintLeagueResponse>> {
    if !space_pk.is_space_key() {
        return Err(Error::InvalidSpacePartitionKey);
    }
    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    SprintLeague::vote(
        &dynamo.client,
        &space_pk,
        &user.pk,
        &req.player_sk,
        req.referral_code,
    )
    .await?;

    Ok(Json(VoteSprintLeagueResponse::default()))
}
