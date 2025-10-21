// use crate::features::spaces::sprint_leagues::CreatePlayerRequest;
use crate::{
    AppState,
    controllers::v3::spaces::dto::SpacePathParam,
    error::Error,
    features::spaces::sprint_leagues::{SprintLeague, SprintLeagueResponse},
    models::{SpaceCommon, User},
    types::{Partition, TeamGroupPermission},
};
use aide::{NoApi, OperationIo};
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, OperationIo, JsonSchema)]
pub struct CreateSprintLeagueRequest {
    // players: Vec<CreatePlayerRequest>,
}

pub async fn create_sprint_league_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(_req): Json<CreateSprintLeagueRequest>,
) -> crate::Result<Json<SprintLeagueResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::SpaceNotFound);
    }

    let (space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let is_editable = space_common.validate_editable();
    if !is_editable {
        return Err(Error::SpaceNotEditable);
    }

    let sprint_league = SprintLeague::new(space_pk.clone())?;

    sprint_league.create(&dynamo.client).await?;

    Ok(Json((sprint_league, vec![], false).into()))
}
