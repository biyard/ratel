use crate::{
    controllers::v3::posts::create_post::CreatePostResponse, tests::v3_setup::TestContextV3,
};

#[tokio::test]
async fn test_create_post() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, headers, body) = crate::post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1,
        response_type: CreatePostResponse,
    };

    assert_eq!(status, 200);
    assert!(body.post_pk.to_string().len() > 0);
}

#[tokio::test]
async fn test_create_post_with_invalid_team() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, headers, body) = crate::post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1,
        body: {
            "team_pk": "TEAM#invalid"
        },
        response_type: serde_json::Value,
    };

    assert_eq!(status, 404);
    assert_eq!(body["code"], 4000);
}

#[tokio::test]
async fn test_post_like() {}
