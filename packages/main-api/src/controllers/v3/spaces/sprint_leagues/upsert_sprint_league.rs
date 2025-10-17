use super::dto::SprintLeagueResponse;
use crate::{
    AppState,
    controllers::v3::spaces::dto::SpacePathParam,
    error::Error,
    models::{SpaceCommon, SprintLeague, SprintLeaguePlayer, User},
    types::{EntityType, Partition, TeamGroupPermission},
};
use aide::{NoApi, OperationIo};
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;
use serde::Deserialize;

use super::dto::CreatePlayerRequest;

#[derive(Debug, Deserialize, Default, OperationIo, JsonSchema)]
pub struct UpsertSprintLeaguePlayerRequest {
    players: Vec<CreatePlayerRequest>,
}

pub async fn upsert_sprint_league_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpsertSprintLeaguePlayerRequest>,
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

    let mut transact_write_items = vec![];
    let sprint_league = if let Some(sprint_league) =
        SprintLeague::get(&dynamo.client, &space_pk, Some(EntityType::SprintLeague)).await?
    {
        SprintLeaguePlayer::delete_all(&dynamo.client, &space_pk).await?;
        sprint_league
    } else {
        let sprint_league = SprintLeague::new(space_pk.clone())?;
        transact_write_items.push(sprint_league.create_transact_write_item());
        sprint_league
    };

    let mut players = Vec::new();
    for player_req in req.players {
        let player = SprintLeaguePlayer::new(
            space_pk.clone(),
            player_req.name,
            player_req.description,
            player_req.player_image,
        )?;
        transact_write_items.push(player.create_transact_write_item());
        players.push(player);
    }

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(transact_write_items))
        .send()
        .await
        .map_err(|e| {
            crate::Error::InternalServerError(format!(
                "Failed to upsert sprint league players: {}",
                e
            ))
        })?;

    Ok(Json(SprintLeagueResponse {
        players,
        votes: sprint_league.votes,
        is_voted: false, // Before Voting Start
    }))
}
