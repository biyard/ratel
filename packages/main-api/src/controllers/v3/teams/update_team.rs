use crate::{
    AppState, Error,
    models::{
        team::Team,
        user::{User, UserTeam, UserTeamQueryOption},
    },
    types::{EntityType, Permissions, TeamGroupPermission},
    utils::{
        security::{RatelResource, check_any_permission_with_user},
        validator::{validate_description, validate_image_url},
    },
};

use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use serde::Deserialize;
use validator::Validate;

use super::dto::TeamResponse;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdateTeamPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_pk: String,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct UpdateTeamRequest {
    #[schemars(description = "Team display name to update")]
    pub nickname: Option<String>,
    #[schemars(description = "Team description to update")]
    #[validate(custom(function = "validate_description"))]
    pub description: Option<String>,
    #[schemars(description = "Team profile URL to update")]
    #[validate(custom(function = "validate_image_url"))]
    pub profile_url: Option<String>,
    #[schemars(description = "Team dao address to update")]
    pub dao_address: Option<String>,
}

pub type UpdateTeamResponse = TeamResponse;

pub async fn update_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<User>,
    NoApi(perm): NoApi<Permissions>,
    Path(params): Path<UpdateTeamPathParams>,
    Json(req): Json<UpdateTeamRequest>,
) -> Result<Json<UpdateTeamResponse>, Error> {
    perm.permitted(TeamGroupPermission::TeamEdit)?;

    tracing::debug!("Updating team: {:?}", req);

    let team = Team::get(&dynamo.client, &params.team_pk, Some(EntityType::Team)).await?;
    if team.is_none() {
        return Err(Error::NotFound("Team not found".into()));
    }
    let mut team = team.unwrap();
    let mut need_update_user_team = false;
    let mut updater = Team::updater(&params.team_pk, EntityType::Team);
    if let Some(nickname) = req.nickname {
        updater = updater.with_display_name(nickname.clone());
        team.display_name = nickname;
        need_update_user_team = true;
    }
    if let Some(description) = req.description {
        updater = updater.with_description(description.clone());
        team.description = description;
        need_update_user_team = true;
    }
    if let Some(profile_url) = req.profile_url {
        updater = updater.with_profile_url(profile_url.clone());
        team.profile_url = profile_url;
    }
    if let Some(dao_address) = req.dao_address {
        updater = updater.with_dao_address(dao_address.clone());
        team.dao_address = Some(dao_address);
    }

    updater.execute(&dynamo.client).await?;

    if !need_update_user_team {
        return Ok(Json(team.into()));
    }

    let mut bookmark: Option<String> = None;
    let user_team_sk = EntityType::UserTeam(params.team_pk);
    loop {
        let mut option = UserTeamQueryOption::builder();
        if let Some(b) = &bookmark {
            option = option.bookmark(b.clone());
        }
        let (user_teams, next) =
            UserTeam::find_by_team(&dynamo.client, &user_team_sk, option).await?;
        tracing::debug!("Found {:?} user teams", user_teams);
        for user_team in user_teams {
            let mut updater = UserTeam::updater(&user_team.pk, &user_team.sk)
                .with_display_name(team.display_name.clone())
                .with_profile_url(team.profile_url.clone());

            if team.dao_address.clone().is_some() {
                updater = updater.with_dao_address(team.dao_address.clone().unwrap());
            }

            updater.execute(&dynamo.client).await?;
        }
        if next.is_none() {
            break;
        }
        bookmark = next;
    }

    Ok(Json(team.into()))
}
