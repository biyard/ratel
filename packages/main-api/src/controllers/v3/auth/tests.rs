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

    let (status, headers, user) = post_with_body! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email,
            "password": "0x1111",
            "display_name": "testuser",
            "username": username,
            "profile_url": "https://example.com/profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        },
        response_type: crate::models::user::User,
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert_eq!(user.username, username);
    assert_eq!(user.email, email);

    let (status, headers, user) = post_with_body! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email,
            "password": "0x1111",
        },
        response_type: crate::models::user::User,
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert_eq!(user.username, username);
    assert_eq!(user.email, email);
}
