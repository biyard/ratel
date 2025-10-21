use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::sprint_leagues::CreatePlayerRequest;
use crate::{
    AppState,
    error::Error,
    features::spaces::sprint_leagues::{SprintLeague, SprintLeaguePlayer},
    models::{SpaceCommon, User},
    types::{EntityType, Partition, TeamGroupPermission},
};
use aide::NoApi;
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

pub async fn create_sprint_league_player_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<CreatePlayerRequest>,
) -> crate::Result<Json<SprintLeaguePlayer>> {
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
    let player = SprintLeaguePlayer::new(
        space_pk.clone(),
        req.name,
        req.description,
        req.player_image,
    )?;

    let sprint_league = SprintLeague::updater(&space_pk, EntityType::SprintLeague)
        .increase_players(1)
        .transact_write_item();

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(vec![
            sprint_league,
            player.create_transact_write_item(),
        ]))
        .send()
        .await
        .map_err(|e| {
            crate::Error::InternalServerError(format!(
                "Failed to upsert sprint league players: {}",
                e
            ))
        })?;

    Ok(Json(player))
}
