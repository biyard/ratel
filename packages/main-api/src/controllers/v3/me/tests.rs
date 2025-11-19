use crate::controllers::v3::me::update_notification_status::UpdateMyNotificationsStatusResponse;
use crate::controllers::v3::me::update_user::{UpdateUserRequest, update_user_handler};
use crate::controllers::v3::posts::create_post::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::members::UpsertInvitationResponse;
use crate::features::notification::NotificationResponse;
use crate::tests::v3_setup::{TestContextV3, setup_v3};
use crate::tests::{create_nick_name, create_user_name, create_user_session};
use crate::types::notification_status::NotificationStatus;
use crate::{
    tests::{create_app_state, create_test_user},
    types::Theme,
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{Json, extract::State},
};
use validator::ValidateLength;

use crate::controllers::v3::teams::{
    create_team::{CreateTeamRequest, create_team_handler},
    get_team::{GetTeamPathParams, get_team_handler},
};
use crate::*;
use by_axum::axum::extract::Path;

#[tokio::test]
async fn test_update_user_with_team_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let username = create_user_name();
    let team_display_name = format!("test_team_{}", username);
    let team_username = format!("test_username_{}", username);

    // Create Team
    let create_res = create_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Json(CreateTeamRequest {
            nickname: team_display_name.clone(),
            username: team_username.clone(),
            description: "This is a test team".into(),
            profile_url: "https://metadata.ratel.foundation/ratel/default-profile.png".into(),
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
        NoApi(Some(user.clone())),
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

    // Use the team_pk directly (it's already a Partition)
    let team = get_team_handler(
        State(app_state.clone()),
        NoApi(Some(user.clone())),
        Path(GetTeamPathParams {
            team_pk: team.team_pk.to_string(),
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

    let new_theme = if user.theme == Theme::Light {
        Theme::Dark
    } else {
        Theme::Light
    };

    let res = update_user_handler(
        State(app_state),
        NoApi(Some(user)),
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

#[tokio::test]
async fn test_list_my_posts() {
    let TestContextV3 {
        app,
        test_user: (_, headers),
        ..
    } = setup_v3().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    tracing::info!("Create post response pk: {:?}", create_body.post_pk);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: headers.clone(),
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();
    let _images = vec!["https://example.com/image1.png".to_string()];

    let title = "Updated Title";
    let content = "<p>Updated Content</p>";

    let path = format!("/v3/posts/{}", post_pk.to_string());

    // Writing
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: headers.clone(),
        body: {
            "title": title,
            "content": content
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["title"], title);
    assert_eq!(body["html_contents"], content);

    // Info
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: headers.clone(),
        body: {
            "visibility": "PUBLIC"
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["visibility"], "PUBLIC");

    // Publish
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: headers.clone(),
        body: {
            "title": title,
            "content": title,
            "publish": true
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["status"], 2);

    // List My Posts
    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/me/posts",
        headers: headers.clone(),
    };

    assert_eq!(status, 200);
    assert!(
        body["items"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or_default()
            > 0,
        "No posts found"
    );
}

#[tokio::test]
async fn test_list_my_notifications() {
    let TestContextV3 {
        ddb,
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let (_status, _headers, post) = post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse
    };

    let feed_pk = post.post_pk.clone();

    let (_status, _headers, space) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": SpaceType::Deliberation,
            "post_pk": feed_pk
        },
        response_type: CreateSpaceResponse
    };

    let space_pk = space.space_pk.clone();

    let (new_user, _headers) = create_user_session(app.clone(), &ddb).await;
    let (new_user_2, _headers) = create_user_session(app.clone(), &ddb).await;
    let (new_user_3, _headers) = create_user_session(app.clone(), &ddb).await;

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/members", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![user.clone().pk, new_user.clone().pk, new_user_2.clone().pk, new_user_3.clone().pk],
            "removed_user_pks": Vec::<Partition>::new()
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 200);

    let (status, _, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/me/notifications",
        headers: headers.clone(),
        response_type: ListItemsResponse<NotificationResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);

    let created_at = body.items[0].created_at;

    let (status, _headers, body_status_unread) = get! {
        app: app,
        path: "/v3/me/notifications?status=Unread",
        headers: headers.clone(),
        response_type: ListItemsResponse<NotificationResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body_status_unread.items.len(), 1);

    let (status, _headers, body_status_read) = get! {
        app: app,
        path: "/v3/me/notifications?status=Read",
        headers: headers.clone(),
        response_type: ListItemsResponse<NotificationResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body_status_read.items.len(), 0);

    let since_before = created_at - 1;
    let (status, _headers, body_since_before) = get! {
        app: app,
        path: format!("/v3/me/notifications?since={}", since_before),
        headers: headers.clone(),
        response_type: ListItemsResponse<NotificationResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body_since_before.items.len(), 1);

    let since_after = created_at + 1;
    let (status, _headers, body_since_after) = get! {
        app: app,
        path: format!("/v3/me/notifications?since={}", since_after),
        headers: headers.clone(),
        response_type: ListItemsResponse<NotificationResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body_since_after.items.len(), 0);
}

#[tokio::test]
async fn test_update_notification_status() {
    let TestContextV3 {
        ddb,
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let (_status, _headers, post) = post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse
    };

    let feed_pk = post.post_pk.clone();

    let (_status, _headers, space) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": SpaceType::Deliberation,
            "post_pk": feed_pk
        },
        response_type: CreateSpaceResponse
    };

    let space_pk = space.space_pk.clone();

    let (new_user, _h1) = create_user_session(app.clone(), &ddb).await;
    let (new_user_2, _h2) = create_user_session(app.clone(), &ddb).await;
    let (new_user_3, _h3) = create_user_session(app.clone(), &ddb).await;

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/members", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "new_user_pks": vec![
                user.clone().pk,
                new_user.clone().pk,
                new_user_2.clone().pk,
                new_user_3.clone().pk,
            ],
            "removed_user_pks": Vec::<Partition>::new()
        },
        response_type: UpsertInvitationResponse
    };

    assert_eq!(status, 200);

    let (status, _, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "publish": true,
            "visibility": "PRIVATE",
        }
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/me/notifications",
        headers: headers.clone(),
        response_type: ListItemsResponse<NotificationResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);

    let noti = &body.items[0];
    assert_eq!(noti.status, NotificationStatus::Unread);
    let created_at = noti.created_at;

    let (status, _headers, update_res) = patch! {
        app: app,
        path: "/v3/me/notifications",
        headers: headers.clone(),
        body: {
            "notification": "notification_time",
            "time": created_at
        },
        response_type: UpdateMyNotificationsStatusResponse
    };

    assert_eq!(status, 200, "failed to update notification status");
    assert_eq!(
        update_res.updated, 1,
        "expected 1 notification to be updated, got {}",
        update_res.updated
    );

    let (status, _headers, body_after) = get! {
        app: app,
        path: "/v3/me/notifications",
        headers: headers.clone(),
        response_type: ListItemsResponse<NotificationResponse>
    };

    assert_eq!(status, 200, "failed to list my notifications after update");
    assert_eq!(body_after.items.len(), 1);

    let noti_after = &body_after.items[0];
    assert_eq!(noti_after.status, NotificationStatus::Read);
}
#[tokio::test]
async fn test_list_my_drafts() {
    let TestContextV3 {
        app,
        test_user: (_, headers),
        ..
    } = setup_v3().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: headers.clone(),
    };
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());
    assert_eq!(body["post"]["status"], 1);

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();
    let _images = vec!["https://example.com/image1.png".to_string()];

    let title = "Draft title";
    let content = "<p>draft Content</p>";

    let path = format!("/v3/posts/{}", post_pk.to_string());

    // Writing
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: headers.clone(),
        body: {
            "title": title,
            "content": content
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["title"], title, "feed: {}", create_body.post_pk);
    assert_eq!(
        body["html_contents"], content,
        "feed: {}",
        create_body.post_pk
    );

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: headers.clone(),
    };
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());
    assert_eq!(body["post"]["status"], 1, "feed: {}", create_body.post_pk);

    // Info
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: headers.clone(),
        body: {
            "visibility": "PUBLIC"
        }
    };

    assert_eq!(status, 200, "feed: {}", create_body.post_pk);
    assert_eq!(
        body["visibility"], "PUBLIC",
        "feed: {}",
        create_body.post_pk
    );

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: headers.clone(),
    };
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());
    assert_eq!(body["post"]["status"], 1, "feed: {}", create_body.post_pk);

    // List My Drafts
    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/me/drafts",
        headers: headers.clone(),
    };

    assert_eq!(status, 200, "feed: {}", create_body.post_pk);
    assert!(
        body["items"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or_default()
            > 0,
        "No posts found: {}",
        create_body.post_pk
    );
    assert_eq!(
        body["items"][0]["pk"], post_pk,
        "feed: {}",
        create_body.post_pk
    );
}
