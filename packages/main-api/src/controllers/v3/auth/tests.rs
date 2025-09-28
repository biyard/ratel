#![allow(warnings)]

use crate::{
    models::user::User,
    post_with_body,
    tests::v3_setup::{TestContextV3, setup_v3},
};
use bdk::prelude::*;
use ethers::providers::StreamExt;

#[tokio::test]
async fn test_email_with_password_signup() {
    let TestContextV3 { app, now, .. } = setup_v3().await;

    let email = format!("testuser{}@example.com", now);
    let username = format!("testuser{}", now);

    // let (status, headers, user) = post_with_body! {
    //     app: app,
    //     path: "/v3/auth/login",
    //     body: {
    //         "email": email,
    //         "password": "0x1111",
    //         "display_name": "testuser",
    //         "username": username,
    //         "profile_url": "https://example.com/profile.png",
    //         "description": "This is a test user.",
    //         "term_agreed": true,
    //         "informed_agreed": true,
    //     },
    //     response_type: crate::models::user::User,
    // };

    let (status, headers, user) = {
        use bdk::prelude::by_axum::axum;

        let req = axum::http::Request::builder()
            .uri(concat!("http://localhost:3000", "/v3/auth/login"))
            .method("POST")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(
                serde_json::to_vec(&serde_json::json!({
                    "email": email,
                    "password": "0x1111",
                    "display_name": "testuser",
                    "username": username,
                    "profile_url": "https://example.com/profile.png",
                    "description": "This is a test user.",
                    "term_agreed": true,
                    "informed_agreed": true,

                }))
                .unwrap(),
            ))
            .unwrap();

        let res: axum::http::Response<axum::body::Body> =
            tower::ServiceExt::oneshot(app.clone(), req).await.unwrap();

        let (parts, body) = res.into_parts();
        let body_bytes = axum::body::to_bytes(body, 10 * 1024 * 1024).await.unwrap();
        tracing::info!("Response body: {:?}", std::str::from_utf8(&body_bytes));

        let parsed: crate::models::user::User = serde_json::from_slice(&body_bytes).unwrap();

        (parts.status, parts.headers, parsed)
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    // assert_eq!(user.nickname, "testuser");
}
