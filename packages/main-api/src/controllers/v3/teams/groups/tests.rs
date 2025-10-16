#![allow(warnings)]
use crate::{
    controllers::v3::{
        me::get_info::get_info_handler,
        teams::{
            create_team::{CreateTeamRequest, create_team_handler},
            get_team::{GetTeamPathParams, get_team_handler},
            groups::{
                add_member::{AddMemberPathParams, AddMemberRequest, add_member_handler},
                create_group::{CreateGroupPathParams, CreateGroupRequest, create_group_handler},
                update_group::{UpdateGroupPathParams, UpdateGroupRequest, update_group_handler},
            },
        },
    },
    tests::{create_app_state, create_test_user},
    types::TeamGroupPermission,
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
#[tokio::test]
async fn test_update_group_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;

    let team_username = format!("TEAM{}", uuid::Uuid::new_v4().to_string());
    // Create a team
    let team = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Json(CreateTeamRequest {
            username: team_username.clone(),
            nickname: format!("{}'s Team", team_username),
            profile_url: "https://example.com/profile.png".into(),
            description: "This is a test team".into(),
        }),
    )
    .await;

    assert!(team.is_ok(), "Failed to create team: {:?}", team.err());
    let team = team.unwrap().0;

    // Use just the UUID for path params (handlers will add TEAM# prefix)
    let team_uuid = team.team_pk.clone();
    let full_team_pk = format!("TEAM#{}", team_uuid);

    // Create a team group
    let team_group = create_group_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(CreateGroupPathParams {
            team_pk: full_team_pk.clone(),
        }),
        Json(CreateGroupRequest {
            name: "Test Group".into(),
            description: "A group for testing".into(),
            image_url: "https://example.com/image.png".into(),
            permissions: vec![TeamGroupPermission::GroupEdit],
        }),
    )
    .await;
    assert!(
        team_group.is_ok(),
        "Failed to create team group: {:?}",
        team_group.err()
    );

    let team_group = team_group.unwrap().0;

    let res = update_group_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(UpdateGroupPathParams {
            team_pk: full_team_pk.clone(),
            group_sk: team_group.group_sk.clone(),
        }),
        Json(UpdateGroupRequest {
            name: Some("Updated Group Name".into()),
            description: Some("Updated description".into()),
            permissions: Some(vec![
                TeamGroupPermission::GroupEdit,
                TeamGroupPermission::TeamEdit,
            ]),
        }),
    )
    .await;

    assert!(res.is_ok(), "Failed to update group: {:?}", res.err());
}
#[tokio::test]
async fn test_update_with_permisison() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;

    let team_username = format!("TEAM{}", uuid::Uuid::new_v4().to_string());
    // Create a team
    let team = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Json(CreateTeamRequest {
            username: team_username.clone(),
            nickname: format!("{}'s Team", team_username),
            profile_url: "https://example.com/profile.png".into(),
            description: "This is a test team".into(),
        }),
    )
    .await;

    assert!(team.is_ok(), "Failed to create team: {:?}", team.err());
    let team = team.unwrap().0;

    // Use just the UUID for path params (handlers will add TEAM# prefix)
    let team_uuid = team.team_pk.clone();
    let full_team_pk = format!("TEAM#{}", team_uuid);

    // Create a team group
    let team_group = create_group_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(CreateGroupPathParams {
            team_pk: full_team_pk.clone(),
        }),
        Json(CreateGroupRequest {
            name: "Test Group".into(),
            description: "A group for testing".into(),
            image_url: "https://example.com/image.png".into(),
            permissions: vec![TeamGroupPermission::GroupEdit],
        }),
    )
    .await;
    assert!(
        team_group.is_ok(),
        "Failed to create team group: {:?}",
        team_group.err()
    );

    let team_group = team_group.unwrap().0;

    let user2 = create_test_user(&cli).await;

    let res = add_member_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(AddMemberPathParams {
            team_pk: full_team_pk.clone(),
            group_sk: team_group.group_sk.clone(),
        }),
        Json(AddMemberRequest {
            user_pks: vec![user2.pk.to_string()],
        }),
    )
    .await;
    assert!(
        res.is_ok(),
        "Failed to add member to group: {:?}",
        res.err()
    );
    let res = res.unwrap().0;

    assert!(
        res.total_added == 1,
        "Expected total_added to be 1 but got: {:?}",
        res.total_added
    );

    // Try to update permission with user2 (should fail)
    let res = update_group_handler(
        State(app_state.clone()),
        NoApi(Some(user2.clone())),
        Path(UpdateGroupPathParams {
            team_pk: full_team_pk.clone(),
            group_sk: team_group.group_sk.clone(),
        }),
        Json(UpdateGroupRequest {
            name: Some("Updated Group Name".into()),
            description: Some("Updated description".into()),
            permissions: Some(vec![
                TeamGroupPermission::GroupEdit,
                TeamGroupPermission::TeamEdit,
            ]),
        }),
    )
    .await;
    assert!(
        res.is_err(),
        "Expected error reason: without Permission but got: {:?}",
        res.ok()
    );
    // Update permission with user2 (should succeed)
    let res = update_group_handler(
        State(app_state.clone()),
        NoApi(Some(user2.clone())),
        Path(UpdateGroupPathParams {
            team_pk: full_team_pk.clone(),
            group_sk: team_group.group_sk.clone(),
        }),
        Json(UpdateGroupRequest {
            name: Some("Updated Group Name".into()),
            description: Some("Updated description".into()),
            permissions: None, // No permission change
        }),
    )
    .await;

    assert!(res.is_ok(), "Failed to update group: {:?}", res.err());
}
#[tokio::test]
async fn test_add_member_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let team_username = format!("TEAM{}", uuid::Uuid::new_v4().to_string());
    // Create a team
    let team = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Json(CreateTeamRequest {
            username: team_username.clone(),
            nickname: format!("{}'s Team", team_username),
            profile_url: "https://example.com/profile.png".into(),
            description: "This is a test team".into(),
        }),
    )
    .await;

    assert!(team.is_ok(), "Failed to create team: {:?}", team.err());
    let team = team.unwrap().0;

    // Use just the UUID for path params (handlers will add TEAM# prefix)
    let team_uuid = team.team_pk.clone();
    // Full PK for internal operations
    let full_team_pk = format!("TEAM#{}", team_uuid);

    // Create a team group
    let team_group = create_group_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(CreateGroupPathParams {
            team_pk: full_team_pk.clone(),
        }),
        Json(CreateGroupRequest {
            name: "Test Group".into(),
            description: "A group for testing".into(),
            image_url: "https://example.com/image.png".into(),
            permissions: vec![TeamGroupPermission::GroupEdit],
        }),
    )
    .await;
    assert!(
        team_group.is_ok(),
        "Failed to create team group: {:?}",
        team_group.err()
    );

    let team_group = team_group.unwrap().0;

    // Create Some users to be added
    let user2 = create_test_user(&cli).await;
    let user3 = create_test_user(&cli).await;

    // Call add_member_handler
    let add_member_res = add_member_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(AddMemberPathParams {
            team_pk: full_team_pk.clone(),
            group_sk: team_group.group_sk.clone(),
        }),
        Json(AddMemberRequest {
            user_pks: vec![user2.pk.to_string(), user3.pk.to_string()],
        }),
    )
    .await;

    assert!(
        add_member_res.is_ok(),
        "Failed to add members: {:?}",
        add_member_res.err()
    );

    let team = get_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(GetTeamPathParams {
            team_pk: full_team_pk.clone(),
        }),
    )
    .await;

    assert!(
        team.is_ok(),
        "Failed to get team after adding members: {:?}",
        team.err()
    );

    let team = team.unwrap().0;
    let res = team.groups.unwrap_or_default();
    let team_group = res
        .into_iter()
        .find(|g| {
            // Compare the UUID part of sk (format: "TEAM_GROUP#uuid") with group_sk (just uuid)
            g.sk.split('#').last().unwrap_or("") == team_group.group_sk
        })
        .expect("Team group should exist");

    assert_eq!(
        team_group.members, 3,
        "Team group members should be 3(Owner + 2 added)"
    );

    // FIXME: Use oneshot and session
    // let user2 = get_info_handler(State(app_state.clone()), Extension(Some(auth2.clone()))).await;

    // assert!(user2.is_ok(), "Failed to get user2 info: {:?}", user2.err());
    // let user2 = user2.unwrap().0;
    // let user2_teams = user2.teams.unwrap_or_default();

    // assert_eq!(user2_teams.len(), 1, "User2 should be in 1 team");
}
