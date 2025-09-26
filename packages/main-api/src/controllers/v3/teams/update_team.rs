use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamResponse},
        user::{UserTeam, UserTeamQueryOption},
    },
    types::{EntityType, TeamGroupPermission},
    utils::{
        security::{RatelResource, check_any_permission},
        validator::{validate_description, validate_image_url, validate_nickname},
    },
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdateTeamPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_pk: String,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct UpdateTeamRequest {
    #[schemars(description = "Team display name to update")]
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: Option<String>,
    #[schemars(description = "Team description to update")]
    #[validate(custom(function = "validate_description"))]
    pub description: Option<String>,
    #[schemars(description = "Team profile URL to update")]
    #[validate(custom(function = "validate_image_url"))]
    pub profile_url: Option<String>,
}

pub type UpdateTeamResponse = TeamResponse;

pub async fn update_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<UpdateTeamPathParams>,
    Json(req): Json<UpdateTeamRequest>,
) -> Result<Json<UpdateTeamResponse>, Error2> {
    check_any_permission(
        &dynamo.client,
        auth,
        RatelResource::Team {
            team_pk: params.team_pk.clone(),
        },
        vec![
            TeamGroupPermission::TeamEdit,
            TeamGroupPermission::TeamAdmin,
        ],
    )
    .await?;
    tracing::debug!("Updating team: {:?}", req);

    let team = Team::get(&dynamo.client, &params.team_pk, Some(EntityType::Team)).await?;
    if team.is_none() {
        return Err(Error2::NotFound("Team not found".into()));
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
            UserTeam::updater(&user_team.pk, &user_team.sk)
                .with_display_name(team.display_name.clone())
                .with_profile_url(team.profile_url.clone())
                .execute(&dynamo.client)
                .await?;
        }
        if next.is_none() {
            break;
        }
        bookmark = next;
    }

    Ok(Json(team.into()))
}
