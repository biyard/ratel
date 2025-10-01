use crate::{
    controllers::v3::posts::create_post::CreatePostResponse, tests::v3_setup::TestContextV3,
};

#[tokio::test]
async fn test_create_post() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    let (status, headers, body) = crate::post! {
        app: app,
        path: "/v3/posts",
        response_type: CreatePostResponse,
    };
}

#[tokio::test]
async fn test_post_like() {}
