#![allow(warnings)]
use crate::{
    models::email::EmailVerification,
    post,
    tests::v3_setup::{TestContextV3, setup_v3},
    types::{EntityType, Partition},
};

#[tokio::test]
async fn test_verification_code() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("vc+{now}@ratel.foundation");
    tracing::info!("Sending verification code to {}", email);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "email": email.clone(),
        },
        response_type: super::send_code::SendCodeResponse,
    };
    assert_eq!(status, 200);
    assert!(body.expired_at > now as i64);

    let verification = EmailVerification::get(
        &ddb,
        Partition::Email(email.clone()),
        Some(EntityType::EmailVerification),
    )
    .await
    .unwrap()
    .unwrap();
    let code = verification.value;
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/verify-code",
        body: {
            "email": email.clone(),
            "code": code,
        },
        response_type: super::verify_code::VerifyCodeResponse,
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);
}

#[tokio::test]
async fn test_verification_invalid_code() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("vc1+{now}@ratel.foundation");
    tracing::info!("Sending verification code to {}", email);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "email": email.clone(),
        },
        response_type: super::send_code::SendCodeResponse,
    };
    assert_eq!(status, 200);
    assert!(body.expired_at > now as i64);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/verify-code",
        body: {
            "email": email.clone(),
            "code": "111",
        }
    };

    let verification = EmailVerification::get(
        &ddb,
        Partition::Email(email.clone()),
        Some(EntityType::EmailVerification),
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(status, 400);
    assert_eq!(verification.attempt_count, 1);
}
