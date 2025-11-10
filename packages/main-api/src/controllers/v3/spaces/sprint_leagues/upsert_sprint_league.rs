use crate::features::spaces::sprint_leagues::CreatePlayerRequest;
use crate::{
    AppState,
    controllers::v3::spaces::dto::SpacePathParam,
    error::Error,
    features::spaces::sprint_leagues::{SprintLeague, SprintLeaguePlayer, SprintLeagueResponse},
    models::{SpaceCommon, User},
    types::{EntityType, Partition, TeamGroupPermission},
};
use aide::{NoApi, OperationIo};
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;
use serde::Deserialize;
use crate::types::Permissions;

#[derive(Debug, Deserialize, Default, OperationIo, JsonSchema)]
pub struct UpsertSprintLeaguePlayerRequest {
    players: Vec<CreatePlayerRequest>,
}

pub async fn upsert_sprint_league_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpsertSprintLeaguePlayerRequest>,
) -> crate::Result<Json<SprintLeagueResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::SpaceNotFound);
    }

    if req.players.len() != 3 {
        return Err(Error::InvalidSprintLeaguePlayer);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon)).await?
        .ok_or(Error::SpaceNotFound)?;

    let is_editable = space_common.validate_editable();
    if !is_editable {
        return Err(Error::SpaceNotEditable);
    }

    let mut transact_write_items = vec![];

    let sprint_league = if let Some(sprint_league) =
        SprintLeague::get(&dynamo.client, &space_pk, Some(EntityType::SprintLeague)).await?
    {
        SprintLeaguePlayer::delete_all(&dynamo.client, &space_pk).await?;
        let updater = SprintLeague::updater(&space_pk, EntityType::SprintLeague)
            .with_players(req.players.len() as i64)
            .transact_write_item();
        transact_write_items.push(updater);
        sprint_league
    } else {
        let sprint_league =
            SprintLeague::new(space_pk.clone())?.with_players(req.players.len() as i64);

        transact_write_items.push(sprint_league.create_transact_write_item());

        sprint_league
    };

    let mut players = Vec::new();
    for (index, player_req) in req.players.into_iter().enumerate() {
        let player = SprintLeaguePlayer::new(
            space_pk.clone(),
            index as i64,
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
                "Failed to upsert sprint league players: {:?}",
                e
            ))
        })?;

    let is_voted = SprintLeague::is_voted(&dynamo.client, &space_pk, &user.pk).await?;

    Ok(Json((sprint_league, players, is_voted).into()))
}
