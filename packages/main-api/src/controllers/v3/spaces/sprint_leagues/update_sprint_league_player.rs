use crate::Error;
use crate::features::spaces::sprint_leagues::{PlayerImage, SprintLeaguePlayerPathParam};
use crate::{
    AppState,
    features::spaces::sprint_leagues::SprintLeaguePlayer,
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
pub struct UpdateSprintLeaguePlayerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub player_image: Option<PlayerImage>,
}

pub async fn update_sprint_league_player_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SprintLeaguePlayerPathParam {
        space_pk,
        player_sk,
    }): Path<SprintLeaguePlayerPathParam>,
    Json(req): Json<UpdateSprintLeaguePlayerRequest>,
) -> crate::Result<Json<SprintLeaguePlayer>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::SpaceNotFound);
    }

    let (space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceWrite,
    )
    .await?;

    if !has_perm {
        return Err(Error::NoPermission);
    }

    let is_editable = space_common.validate_editable();
    if !is_editable {
        return Err(Error::SpaceNotEditable);
    }

    let mut player = SprintLeaguePlayer::get(&dynamo.client, &space_pk, Some(&player_sk))
        .await?
        .ok_or_else(|| Error::InvalidSprintLeaguePlayer)?;

    let mut player_updater = SprintLeaguePlayer::updater(&space_pk, &player_sk);

    if let Some(name) = req.name {
        player.name = name.clone();
        player_updater = player_updater.with_name(name);
    }
    if let Some(description) = req.description {
        player.description = description.clone();
        player_updater = player_updater.with_description(description);
    }
    if let Some(player_image) = req.player_image {
        player.player_image = player_image.clone();
        player_updater = player_updater.with_player_image(player_image);
    }

    player_updater.execute(&dynamo.client).await?;

    Ok(Json(player))
}
