use dto::by_axum::axum::{
    Extension, Json,
    extract::{Path, State},
};

use crate::{
    controllers::v3::{
        posts::create_post::{CreatePostRequest, create_post_handler},
        teams::{
            create_team::{CreateTeamRequest, create_team_handler},
            groups::{
                add_member::{AddMemberPathParams, AddMemberRequest, add_member_handler},
                create_group::{CreateGroupPathParams, CreateGroupRequest, create_group_handler},
            },
        },
    },
    tests::dynamo_test::*,
    types::TeamGroupPermission,
};

#[tokio::test]
async fn test_create_post() {
    // Setup
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let user = create_test_user(cli).await;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state),
        Extension(Some(auth)),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);
    // Test code
}

#[tokio::test]
async fn test_create_post_by_team() {
    // Setup
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let user = create_test_user(cli).await;
    let auth = get_auth(&user);

    let username = create_user_name();
    let nickname = create_nick_name();
    let team = create_team_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateTeamRequest {
            username: username.clone(),
            nickname: nickname.clone(),
            profile_url: "".to_string(),
            description: "".to_string(),
        }),
    )
    .await;
    assert!(team.is_ok(), "Failed to create team: {:?}", team);
    let team_pk = team.unwrap().0.team_pk;

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest {
            team_pk: Some(team_pk.clone()),
        }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let user2 = create_test_user(cli).await;
    let auth2 = get_auth(&user2);
    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth2)),
        Json(CreatePostRequest {
            team_pk: Some(team_pk.clone()),
        }),
    )
    .await;
    assert!(
        post.is_err(),
        "Success to create post without permission: {:?}",
        post
    );

    let user3 = create_test_user(cli).await;
    let auth3 = get_auth(&user3);

    let team_group = create_group_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(CreateGroupPathParams {
            team_pk: team_pk.clone(),
        }),
        Json(CreateGroupRequest {
            name: "Test Group".into(),
            description: "A group for testing".into(),
            image_url: "https://example.com/image.png".into(),
            permissions: vec![TeamGroupPermission::PostWrite],
        }),
    )
    .await;
    assert!(
        team_group.is_ok(),
        "Failed to create team group: {:?}",
        team_group.err()
    );

    let team_group = team_group.unwrap().0;

    let res = add_member_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(AddMemberPathParams {
            team_pk: team_pk.clone(),
            group_sk: team_group.group_sk.clone(),
        }),
        Json(AddMemberRequest {
            user_pks: vec![user3.pk.to_string()],
        }),
    )
    .await;

    assert!(res.is_ok(), "Failed to add members: {:?}", res.err());

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth3)),
        Json(CreatePostRequest {
            team_pk: Some(team_pk.clone()),
        }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);
}
