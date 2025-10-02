use crate::controllers::v3::me::update_user::{UpdateUserRequest, update_user_handler};
use crate::tests::v3_setup::{TestContextV3, setup_v3};
use crate::tests::{create_nick_name, create_user_name};
use crate::{
    tests::{create_app_state, create_test_user, get_auth},
    types::Theme,
};
use dto::by_axum::axum::{
    Json,
    extract::{Extension, State},
};

use crate::controllers::v3::teams::{
    create_team::{CreateTeamRequest, create_team_handler},
    get_team::{GetTeamPathParams, get_team_handler},
};
use crate::*;
use dto::by_axum::axum::extract::Path;

#[tokio::test]
async fn test_update_user_with_team_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user);
    let username = create_user_name();
    let team_display_name = format!("test_team_{}", username);
    let team_username = format!("test_username_{}", username);

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

    // Update User
    let new_nickname = create_nick_name();
    println!("New Nickname: {}", new_nickname);
    let new_profile_url = format!("https://new.url/profile_{}.png", new_nickname);
    let new_description = format!("This is {}'s new description", new_nickname);

    // Update Profile
    let update_user_res = update_user_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(UpdateUserRequest::Profile {
            nickname: new_nickname.clone(),
            profile_url: new_profile_url,
            description: new_description,
        }),
    )
    .await;
    assert!(
        update_user_res.is_ok(),
        "Failed to update user {:?}",
        update_user_res.err()
    );

    let team = get_team_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(GetTeamPathParams {
            team_pk: team.team_pk.clone(),
        }),
    )
    .await;
    assert!(team.is_ok(), "Failed to get team {:?}", team.err());
    let team_owner = team.unwrap().0.owner;
    assert!(team_owner.is_some(), "Team owner should exist");
    let team_owner = team_owner.unwrap();
    assert_eq!(
        team_owner.display_name, new_nickname,
        "Team owner display name was not updated"
    );
}

#[tokio::test]
async fn test_update_user_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user);

    let new_theme = if user.theme == Theme::Light {
        Theme::Dark
    } else {
        Theme::Light
    };

    let res = update_user_handler(
        State(app_state),
        Extension(Some(auth)),
        Json(UpdateUserRequest::Theme { theme: new_theme }),
    )
    .await;

    assert!(res.is_ok(), "Failed to update user: {:?}", res.err());
    let updated_user_response = res.unwrap().0;
    let user_detail = updated_user_response.user;

    assert_eq!(user_detail.theme, new_theme as u8, "Theme was not updated.");
}

#[tokio::test]
async fn test_get_user_info() {
    let TestContextV3 {
        app,
        test_user: (_, headers),
        ..
    } = setup_v3().await;

    let (status, _, _) = get! {
        app: app,
        path: "/v3/me"
    };
    assert_eq!(status, 401);

    // Test Create Team With Auth
    let (status, _, _) = get! {
        app: app,
        path: "/v3/me",
        headers: headers

    };
    assert_eq!(status, 200);
}
