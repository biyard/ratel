use crate::{
    controllers::v3::posts::create_post::CreatePostResponse, models::feed::PostDetailResponse,
    tests::v3_setup::TestContextV3,
};

use crate::*;

#[tokio::test]
async fn test_create_post_by_user() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(body.post_pk.to_string().len() > 0);

    tracing::info!("Create post response pk: {:?}", body.post_pk);

    let path = format!("/v3/posts/{}", body.post_pk.to_string()).replace("#", "%23");

    let (status, _headers, body) = get! {
        app: app,
        path: path,
        headers: test_user.1,
        response_type: PostDetailResponse
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_create_post_with_invalid_team() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1,
        body: {
            "team_pk": "TEAM#invalid"
        }
    };

    assert_eq!(status, 404);
    assert_eq!(body["code"], 4000);
}

#[tokio::test]
async fn test_post_like() {}

#[tokio::test]
async fn test_post_delete() {}

#[tokio::test]
async fn test_post_update() {}
