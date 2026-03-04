use super::*;

#[tokio::test]
async fn test_create_post_by_user() {
    let TestContext { app, test_user, .. } = TestContext::setup().await;

    let (status, _headers, body) = crate::test_post! {
        app: app,
        path: "/api/posts",
        headers: test_user.1.clone(),
    };

    assert_eq!(status, 200, "create post response: {:?}", body);
}

#[tokio::test]
async fn test_create_post_without_auth() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _headers, _body) = crate::test_post! {
        app: app,
        path: "/api/posts",
    };

    assert_ne!(status, 200, "unauthenticated request should fail");
}
