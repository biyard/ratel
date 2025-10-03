use validator::ValidateLength;

use crate::{
    controllers::v3::posts::create_post::CreatePostResponse, tests::v3_setup::TestContextV3,
};

use crate::*;

#[tokio::test]
async fn test_create_post_by_user() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    tracing::info!("Create post response pk: {:?}", create_body.post_pk);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();
    let images = vec!["https://example.com/image1.png".to_string()];

    let title = "Updated Title";
    let content = "<p>Updated Content</p>";

    let path = format!("/v3/posts/{}", post_pk.to_string());

    // Writing
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: test_user.1.clone(),
        body: {
            "title": title,
            "content": content
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["title"], title);
    assert_eq!(body["html_contents"], content);

    // Images
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: test_user.1.clone(),
        body: {
            "images": images
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["urls"].as_array().length().unwrap_or_default(), 1);
    assert_eq!(body["urls"][0], images[0]);

    // Info
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: test_user.1.clone(),
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
        headers: test_user.1.clone(),
        body: {
            "publish": true
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["status"], 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], post_pk);
}

#[tokio::test]
async fn test_block_read_draft_post_from_guest() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(body.post_pk.to_string().len() > 0);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", body.post_pk.to_string()),
    };
    assert_eq!(status, 401);
    assert_eq!(body["code"], 403);
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
async fn test_post_like() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    tracing::info!("Create post response pk: {:?}", create_body.post_pk);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/posts/{}/likes", post_pk),
        headers: test_user.1.clone(),
        body: { "like": true }
    };
    assert_eq!(status, 200);
    assert_eq!(body["like"], true);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: serde_json::Value,
    };

    assert_eq!(status, 200);
    assert_eq!(body["post"]["likes"], 1);
}

#[tokio::test]
async fn test_delete_draft() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    tracing::info!("Create post response pk: {:?}", create_body.post_pk);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();
}

#[tokio::test]
async fn test_delete_post() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    tracing::info!("Create post response pk: {:?}", create_body.post_pk);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        body: {
            "publish": true
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["status"], 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body["post"]["pk"], post_pk);

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone()
    };
    tracing::info!("Get post response: {:?}", body);
    assert_eq!(status, 404);
    assert_eq!(body["code"], 107);
}

#[tokio::test]
async fn test_delete_post_by_guest() {
    // TODO: failure test
}

#[tokio::test]
async fn test_delete_post_by_other_no_permitted() {
    // TODO:  failure test
}

#[tokio::test]
async fn test_delete_post_by_other_permitted() {
    // TODO:  success test
}
