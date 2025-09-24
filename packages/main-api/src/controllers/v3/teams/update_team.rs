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
#[serde(rename_all = "camelCase")]x
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
#[cfg(test)]
pub mod update_team_tests {
    use super::*;
    use crate::controllers::v3::teams::create_team::{CreateTeamRequest, create_team_handler};
    use crate::controllers::v3::teams::get_team::{GetTeamPathParams, get_team_handler};
    use crate::tests::{create_app_state, create_auth, create_user_name, get_test_user};
    #[tokio::test]
    async fn test_update_without_permission() {
        let app_state = create_app_state();
        let user = get_test_user(app_state.clone()).await;
        let auth = create_auth(user.clone()).await;
        let username = create_user_name();
        let team = create_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Json(CreateTeamRequest {
                nickname: format!("team_{}", username),
                username: format!("test_username_{}", username),
                description: "This is a test team".into(),
                profile_url: "https://example.com/profile.png".into(),
            }),
        )
        .await;
        assert!(team.is_ok(), "Failed to create team {:?}", team.err());
        let team = team.unwrap().0;
        let team_pk = team.team_pk;

        let another_user = get_test_user(app_state.clone()).await;
        let another_auth = create_auth(another_user.clone()).await;

        let res = update_team_handler(
            State(app_state.clone()),
            Extension(Some(another_auth)),
            Path(UpdateTeamPathParams {
                team_pk: team_pk.clone(),
            }),
            Json(UpdateTeamRequest {
                nickname: Some("updated_team".into()),
                description: Some("This is an updated test team".into()),
                profile_url: Some("https://example.com/updated_profile.png".into()),
            }),
        )
        .await;
        assert!(res.is_err(), "Update should fail without permission");

        let res = update_team_handler(
            State(app_state),
            Extension(None),
            Path(UpdateTeamPathParams {
                team_pk: team_pk.clone(),
            }),
            Json(UpdateTeamRequest {
                nickname: Some("updated_team".into()),
                description: Some("This is an updated test team".into()),
                profile_url: Some("https://example.com/updated_profile.png".into()),
            }),
        )
        .await;
        assert!(res.is_err(), "Update should fail without auth");
    }

    #[tokio::test]
    async fn test_update_team_handler() {
        use crate::tests::{create_app_state, create_auth, get_test_user};

        let app_state = create_app_state();
        let user = get_test_user(app_state.clone()).await;
        let auth = create_auth(user.clone()).await;
        let username = create_user_name();
        let team_username = format!("team_{}", username);
        let team_display_name = format!("team_{}", username);

        // Create Team
        let create_res = create_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Json(CreateTeamRequest {
                nickname: team_display_name.clone(),
                username: team_username.clone(),
                description: "This is a test team".into(),
                profile_url: "https://example.com/profile.png".into(),
            }),
        )
        .await;
        assert!(
            create_res.is_ok(),
            "Failed to create team {:?}",
            create_res.err()
        );
        let team = create_res.unwrap().0;
        let team_pk = team.team_pk;

        // Update Team
        let new_team_display_name = format!("updated_team_{}", username);
        let new_team_description = "This is an updated test team".to_string();
        let new_team_profile_url = "https://example.com/updated_profile.png".to_string();
        let update_res = update_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(UpdateTeamPathParams {
                team_pk: team_pk.clone(),
            }),
            Json(UpdateTeamRequest {
                nickname: Some(new_team_display_name.clone()),
                description: Some(new_team_description.clone()),
                profile_url: Some(new_team_profile_url.clone()),
            }),
        )
        .await;
        assert!(
            update_res.is_ok(),
            "Failed to update team {:?}",
            update_res.err()
        );

        // Get Team
        let get_res = get_team_handler(
            State(app_state),
            Extension(Some(auth)),
            Path(GetTeamPathParams { team_pk }),
        )
        .await;
        assert!(get_res.is_ok(), "Failed to get team {:?}", get_res.err());
        let team_detail = get_res.unwrap().0;
        let team = &team_detail.team;

        assert_eq!(
            team.nickname, new_team_display_name,
            "Failed to match team nickname"
        );
    }
}
