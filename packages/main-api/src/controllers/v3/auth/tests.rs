#![allow(warnings)]

use std::convert;

use crate::*;
use crate::{
    models::{email::EmailVerification, user::User},
    tests::v3_setup::{TestContextV3, setup_v3},
    types::{EntityType, Partition},
};
use bdk::prelude::*;
use ethers::providers::StreamExt;

#[tokio::test]
async fn test_email_with_password_signup() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("testuser{}@example.com", now);
    let username = format!("testuser{:x}", now);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "email": email.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);
    assert!(body.expired_at > now as i64);
    let EmailVerification { value: code, .. } = EmailVerification::get(
        &ddb,
        Partition::Email(email.clone()),
        Some(EntityType::EmailVerification),
    )
    .await
    .unwrap()
    .unwrap();

    let (status, headers, user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email,
            "password": "0x1111",
            "code": code,
            "display_name": "testuser",
            "username": username,
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert_eq!(user.username, username);
    assert_eq!(user.email, email);

    let (status, headers, user) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email,
            "password": "0x1111",
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert_eq!(user.username, username);
    assert_eq!(user.email, email);
}

#[tokio::test]
async fn test_email_with_invalid_password() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("testuser1{}@example.com", now);
    let username = format!("testuser1{:x}", now);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "email": email.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);
    assert!(body.expired_at > now as i64);
    let EmailVerification { value: code, .. } = EmailVerification::get(
        &ddb,
        Partition::Email(email.clone()),
        Some(EntityType::EmailVerification),
    )
    .await
    .unwrap()
    .unwrap();

    let (status, headers, user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email,
            "password": "0x1111",
            "code": code,
            "display_name": "testuser",
            "username": username,
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert_eq!(user.username, username);
    assert_eq!(user.email, email);

    let (status, headers, _resp_body) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email,
            "password": "0x11112",
        }
    };

    assert_eq!(status, 401);
}

#[tokio::test]
async fn test_email_with_password_signup_with_invalid_code() {
    let TestContextV3 { app, now, .. } = setup_v3().await;

    let email = format!("testuser3{}@example.com", now);
    let username = format!("testuser3{:x}", now);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "email": email.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);
    assert!(body.expired_at > now as i64);

    let (status, headers, user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email,
            "password": "0x1111",
            "code": "111",
            "display_name": "testuser",
            "username": username,
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        }
    };

    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_reset_password() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("testuser{}@example.com", now);
    let username = format!("testuser{:x}", now);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "email": email.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);
    let EmailVerification { value: code, .. } = EmailVerification::get(
        &ddb,
        Partition::Email(email.clone()),
        Some(EntityType::EmailVerification),
    )
    .await
    .unwrap()
    .unwrap();

    let (status, _headers, _user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email,
            "password": "0x1111",
            "code": code,
            "display_name": "testuser",
            "username": username,
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "email": email.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);
    let EmailVerification { value: code, .. } = EmailVerification::get(
        &ddb,
        Partition::Email(email.clone()),
        Some(EntityType::EmailVerification),
    )
    .await
    .unwrap()
    .unwrap();

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/reset",
        body: {
            "email": email,
            "password": "0x11112",
            "code": code
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);

    let (status, _headers, _resp_body) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email,
            "password": "0x11112",
        }
    };

    assert_eq!(status, 200);
}
