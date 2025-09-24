use dto::by_axum::axum::{
    Json,
    extract::{Extension, Path, State},
};

use crate::{
    controllers::v3::teams::{
        create_team::{CreateTeamRequest, create_team_handler},
        get_team::{GetTeamPathParams, get_team_handler},
        update_team::{UpdateTeamPathParams, UpdateTeamRequest, update_team_handler},
    },
    tests::{create_app_state, create_auth, create_user_name, get_test_user},
};
#[tokio::test]
async fn test_update_team_without_permission() {
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
async fn test_update_team() {
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

#[tokio::test]
async fn test_get_team() {
    let app_state = create_app_state();
    let user = get_test_user(app_state.clone()).await;
    let auth = create_auth(user.clone()).await;
    let now = chrono::Utc::now().timestamp();
    let team_display_name = format!("test_team_{}", now);
    let team_username = format!("test_username_{}", now);

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
    let owner = team_detail.owner.as_ref().expect("Owner should exist");

    assert_eq!(
        team.nickname, team_display_name,
        "Failed to match team nickname"
    );
    assert_eq!(
        team.username, team_username,
        "Failed to match team username"
    );
    assert_eq!(
        owner.user_pk.to_string(),
        user.pk.to_string(),
        "Failed to match `owner pk`"
    );
}
