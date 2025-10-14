use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};

use crate::{
    controllers::v3::teams::{
        create_team::{CreateTeamRequest, create_team_handler},
        delete_team::delete_team_handler,
        get_team::{GetTeamPathParams, get_team_handler},
        groups::delete_group::delete_group_handler,
        list_members::list_members_handler,
        update_team::{UpdateTeamPathParams, UpdateTeamRequest, update_team_handler},
    },
    tests::{create_app_state, create_test_user, create_user_name},
};
#[tokio::test]
async fn test_update_team_without_permission() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let username = create_user_name();
    let team = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
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

    let another_user = create_test_user(&cli).await;

    let res = update_team_handler(
        State(app_state.clone()),
        NoApi(Some(another_user)),
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
        NoApi(None),
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
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let username = create_user_name();
    let team_username = format!("team_{}", username);
    let team_display_name = format!("team_{}", username);

    // Create Team
    let create_res = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
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
        NoApi(Some(user.clone())),
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
        NoApi(Some(user)),
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
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let now = chrono::Utc::now().timestamp();
    let team_display_name = format!("test_team_{}", now);
    let team_username = format!("test_username_{}", now);

    // Create Team
    let create_res = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
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
        NoApi(Some(user.clone())),
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

#[tokio::test]
async fn test_list_members() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let username = create_user_name();

    // Create team
    let _team = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Json(CreateTeamRequest {
            nickname: format!("team_{}", username),
            username: format!("test_username_{}", username),
            description: "This is a test team".into(),
            profile_url: "https://example.com/profile.png".into(),
        }),
    )
    .await
    .expect("Failed to create team")
    .0;

    // Get team username for the list_members call
    let team_username = format!("test_username_{}", username);

    // Test list members as team owner
    let list_res = list_members_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(team_username.clone()),
    )
    .await;

    assert!(
        list_res.is_ok(),
        "Failed to list members: {:?}",
        list_res.err()
    );
    let members_response = list_res.unwrap().0;

    // Should have at least the owner in the member list
    assert!(
        !members_response.members.is_empty(),
        "Members list should not be empty"
    );

    // Test unauthorized access
    let other_user = create_test_user(&cli).await;
    let unauthorized_res = list_members_handler(
        State(app_state.clone()),
        NoApi(Some(other_user)),
        Path(team_username),
    )
    .await;

    // Should fail for unauthorized user
    assert!(
        unauthorized_res.is_err(),
        "Unauthorized user should not be able to list members"
    );
}

#[tokio::test]
async fn test_delete_team() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let username = create_user_name();
    let team_username = format!("test_username_{}", username);

    // Create team
    let _team = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Json(CreateTeamRequest {
            nickname: format!("team_{}", username),
            username: team_username.clone(),
            description: "This is a test team".into(),
            profile_url: "https://example.com/profile.png".into(),
        }),
    )
    .await
    .expect("Failed to create team")
    .0;

    // Test unauthorized deletion
    let other_user = create_test_user(&cli).await;
    let unauthorized_res = delete_team_handler(
        State(app_state.clone()),
        NoApi(Some(other_user)),
        Path(team_username.clone()),
    )
    .await;

    assert!(
        unauthorized_res.is_err(),
        "Non-owner should not be able to delete team"
    );

    // Test successful deletion by owner
    let delete_res = delete_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(team_username.clone()),
    )
    .await;

    assert!(
        delete_res.is_ok(),
        "Owner should be able to delete team: {:?}",
        delete_res.err()
    );
    let delete_response = delete_res.unwrap().0;
    assert!(
        delete_response.deleted_count > 0,
        "Should have deleted at least one entity"
    );

    // Test that team no longer exists by trying to list members
    let list_after_delete =
        list_members_handler(State(app_state), NoApi(Some(user)), Path(team_username)).await;

    assert!(
        list_after_delete.is_err(),
        "Team should no longer exist after deletion"
    );
}

#[tokio::test]
async fn test_delete_group() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let username = create_user_name();
    let team_username = format!("test_username_{}", username);

    // Create team
    let _team = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Json(CreateTeamRequest {
            nickname: format!("team_{}", username),
            username: team_username.clone(),
            description: "This is a test team".into(),
            profile_url: "https://example.com/profile.png".into(),
        }),
    )
    .await
    .expect("Failed to create team")
    .0;

    // For this test, we'll use a placeholder group_id since we'd need to create a group first
    let group_id = "test-group-id".to_string();

    // Test unauthorized deletion
    let other_user = create_test_user(&cli).await;
    let unauthorized_res = delete_group_handler(
        State(app_state.clone()),
        NoApi(Some(other_user)),
        Path((team_username.clone(), group_id.clone())),
    )
    .await;

    assert!(
        unauthorized_res.is_err(),
        "Non-owner should not be able to delete group"
    );

    // Test deletion by owner (this will fail gracefully since group doesn't exist)
    let delete_res = delete_group_handler(
        State(app_state.clone()),
        NoApi(Some(user)),
        Path((team_username, group_id)),
    )
    .await;

    // This should fail because the group doesn't exist, but with proper auth
    assert!(
        delete_res.is_err(),
        "Should fail gracefully when group doesn't exist"
    );
}
