#![allow(warnings)]

use crate::by_axum::router::BiyardRouter;
use crate::controllers::v3::auth::list_accounts::AccountItem;
use std::convert;

use crate::auth::change_account::ChangeAccountResponse;
use crate::auth::list_accounts::ListAccountsResponse;
use crate::auth::login::LoginResponse;
use crate::auth::verification::verify_code::VerifyCodeResponse;
use crate::controllers::v3::auth::verification::send_code::SendCodeResponse;
use crate::models::PhoneVerification;
use crate::models::UserRefreshToken;
use crate::*;
use crate::{
    models::{email::EmailVerification, user::User},
    tests::v3_setup::{TestContextV3, setup_v3},
    types::{EntityType, Partition},
};
use bdk::prelude::*;
use ethers::providers::StreamExt;

#[tokio::test]
async fn test_login_with_device_id_issues_single_refresh_token_per_user_device() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("testuser{}@example.com", now);
    let username = format!("testuser{:x}", now);
    let device_id = format!("device-{}", now);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: { "email": email.clone() },
        response_type: SendCodeResponse
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

    let (status, _headers, user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email.clone(),
            "password": "0x1111",
            "code": code,
            "display_name": "testuser",
            "username": username.clone(),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        },
        response_type: crate::models::user::User
    };
    assert_eq!(status, 200);
    assert_eq!(user.username, username);
    assert_eq!(user.email, email);

    let (status, _headers, login1) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email.clone(),
            "password": "0x1111",
            "device_id": device_id.clone(),
        },
        response_type: LoginResponse
    };
    assert_eq!(status, 200);
    let rt1 = login1.refresh_token.clone().unwrap();
    assert_eq!(login1.user.pk, user.pk);

    let (status, _headers, login2) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email.clone(),
            "password": "0x1111",
            "device_id": device_id.clone(),
        },
        response_type: LoginResponse
    };
    assert_eq!(status, 200);
    let rt2 = login2.refresh_token.clone().unwrap();
    assert_ne!(rt1, rt2);
    assert_eq!(login2.user.pk, user.pk);

    let (rts, _) =
        UserRefreshToken::find_by_device_id(&ddb, &device_id, UserRefreshToken::opt_all())
            .await
            .unwrap();

    let mut mine = rts
        .into_iter()
        .filter(|x| x.pk == user.pk)
        .collect::<Vec<_>>();

    assert_eq!(mine.len(), 1);
    assert_eq!(mine[0].device_id, device_id);
}

#[tokio::test]
async fn test_change_account_with_device_id_and_refresh_token() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("testuser{}@example.com", now);
    let username = format!("testuser{:x}", now);
    let device_id = format!("device-{}", now);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: { "email": email.clone() },
        response_type: SendCodeResponse
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

    let (status, _headers, user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "email": email.clone(),
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

    let (status, _headers, login) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "email": email.clone(),
            "password": "0x1111",
            "device_id": device_id.clone(),
        },
        response_type: LoginResponse
    };
    assert_eq!(status, 200);
    let proof_refresh = login.refresh_token.clone().unwrap();

    let (status, _headers, changed) = post! {
        app: app,
        path: "/v3/auth/change-account",
        body: {
            "device_id": device_id.clone(),
            "user_pk": user.pk.to_string(),
            "refresh_token": proof_refresh.clone(),
        },
        response_type: ChangeAccountResponse
    };
    assert_eq!(status, 200);
    assert_eq!(changed.user.pk, user.pk);
    let new_refresh = changed.refresh_token.clone().unwrap();
    assert_ne!(proof_refresh, new_refresh);

    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/auth/change-account",
        body: {
            "device_id": device_id.clone(),
            "user_pk": user.pk.to_string(),
            "refresh_token": proof_refresh,
        }
    };
    assert_eq!(status, 401);

    let (status, _headers, changed2) = post! {
        app: app,
        path: "/v3/auth/change-account",
        body: {
            "device_id": device_id,
            "user_pk": user.pk.to_string(),
            "refresh_token": new_refresh,
        },
        response_type: ChangeAccountResponse
    };
    assert_eq!(status, 200);
    assert_eq!(changed2.user.pk, user.pk);
    assert!(changed2.refresh_token.is_some());
}

async fn signup_email(
    app: AxumRouter,
    ddb: aws_sdk_dynamodb::Client,
    now: u64,
    email: String,
    username: String,
) -> crate::models::user::User {
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: { "email": email.clone() },
        response_type: crate::controllers::v3::auth::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);

    let crate::models::email::EmailVerification { value: code, .. } =
        crate::models::email::EmailVerification::get(
            &ddb,
            crate::types::Partition::Email(email.clone()),
            Some(crate::types::EntityType::EmailVerification),
        )
        .await
        .unwrap()
        .unwrap();

    let (status, _headers, user) = post! {
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

    user
}

#[tokio::test]
async fn test_list_accounts_with_device_id() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let device_id = format!("device-{}", now);

    let email1 = format!("testuser_a{}@example.com", now);
    let username1 = format!("testuser_a{:x}", now);
    let user1 = signup_email(
        app.clone(),
        ddb.clone(),
        now,
        email1.clone(),
        username1.clone(),
    )
    .await;

    let email2 = format!("testuser_b{}@example.com", now);
    let username2 = format!("testuser_b{:x}", now);
    let user2 = signup_email(
        app.clone(),
        ddb.clone(),
        now,
        email2.clone(),
        username2.clone(),
    )
    .await;

    let (status, _headers, _login1) = post! {
        app: app.clone(),
        path: "/v3/auth/login",
        body: {
            "email": email1.clone(),
            "password": "0x1111",
            "device_id": device_id.clone(),
        },
        response_type: LoginResponse
    };
    assert_eq!(status, 200);

    let (status, _headers, _login2) = post! {
        app: app.clone(),
        path: "/v3/auth/login",
        body: {
            "email": email2.clone(),
            "password": "0x1111",
            "device_id": device_id.clone(),
        },
        response_type: LoginResponse
    };
    assert_eq!(status, 200);

    let (status, _headers, _login1_again) = post! {
        app: app.clone(),
        path: "/v3/auth/login",
        body: {
            "email": email1,
            "password": "0x1111",
            "device_id": device_id.clone(),
        },
        response_type: LoginResponse
    };
    assert_eq!(status, 200);

    let path = format!("/v3/auth/accounts?device_id={}", device_id);

    let (status, _headers, listed) = get! {
        app: app,
        path: &path,
        response_type: ListItemsResponse<AccountItem>
    };
    assert_eq!(status, 200);

    assert_eq!(listed.items.len(), 2);

    let a0 = &listed.items[0];
    let a1 = &listed.items[1];

    assert!(a0.last_login_at >= a1.last_login_at);

    let u0 = a0.user_pk.clone();
    let u1 = a1.user_pk.clone();

    assert_ne!(u0, u1);

    let user1_pk = user1.pk.to_string();
    let user2_pk = user2.pk.to_string();

    assert!(
        (u0.to_string() == user1_pk && u1.to_string() == user2_pk)
            || (u0.to_string() == user2_pk && u1.to_string() == user1_pk)
    );

    if a0.user_pk == user1.pk {
        assert_eq!(a0.username, user1.username);
        assert_eq!(a0.display_name, user1.display_name);
        assert_eq!(a0.profile_url, user1.profile_url);
        assert_eq!(a1.username, user2.username);
    } else {
        assert_eq!(a0.username, user2.username);
        assert_eq!(a1.username, user1.username);
    }

    assert_eq!(a0.revoked, false);
    assert_eq!(a1.revoked, false);
}

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
async fn test_phone_signup() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let phone = format!("+1555{:07}", now % 10000000);
    tracing::info!("Sending verification code to {}", phone);

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "phone": phone.clone(),
        },
        response_type: SendCodeResponse
    };
    assert_eq!(status, 200);
    assert!(body.expired_at > now as i64);

    let verification = PhoneVerification::get(
        &ddb,
        Partition::Phone(phone.clone()),
        Some(EntityType::PhoneVerification),
    )
    .await
    .unwrap()
    .unwrap();
    let code = verification.value;
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/verify-code",
        body: {
            "phone": phone.clone(),
            "code": code.clone(),
        },
        response_type: VerifyCodeResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.success, true);

    let (status, headers, user) = post! {
        app: app,
        path: "/v3/auth/signup",
        body: {
            "phone": phone.clone(),
            "code": code,
            "display_name": "testuser",
            "username": "testuser",
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a test user.",
            "term_agreed": true,
            "informed_agreed": true,
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
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

// FIXME: check this failed testcode logic
#[tokio::test]
async fn test_reset_password() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let email = format!("testreset{}@example.com", now);
    let username = format!("testreset{:x}", now);

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

#[tokio::test]
async fn test_phone_login_new_user_auto_registration() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let phone = format!("+1555{:07}", now % 10000000);

    // Send verification code
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "phone": phone.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);
    assert!(body.expired_at > now as i64);

    // Get the verification code from DynamoDB
    let verification = crate::models::phone::PhoneVerification::get(
        &ddb,
        Partition::Phone(phone.clone()),
        Some(EntityType::PhoneVerification),
    )
    .await
    .unwrap()
    .unwrap();
    let code = verification.value;

    // Login with phone (should auto-register)
    let (status, headers, user) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "phone": phone.clone(),
            "code": code,
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert_eq!(user.display_name, phone);
    assert!(user.username.starts_with("user"));
    assert!(user.email.ends_with("@phone.placeholder"));
}

#[tokio::test]
async fn test_phone_login_existing_user() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let phone = format!("+1555{:07}", now % 10000000 + 100);

    // First login - auto-register
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "phone": phone.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);

    let verification = crate::models::phone::PhoneVerification::get(
        &ddb,
        Partition::Phone(phone.clone()),
        Some(EntityType::PhoneVerification),
    )
    .await
    .unwrap()
    .unwrap();
    let code1 = verification.value;

    let (status, _headers, first_user) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "phone": phone.clone(),
            "code": code1,
        },
        response_type: crate::models::user::User
    };
    assert_eq!(status, 200);

    // Second login - should return same user
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "phone": phone.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);

    let verification = crate::models::phone::PhoneVerification::get(
        &ddb,
        Partition::Phone(phone.clone()),
        Some(EntityType::PhoneVerification),
    )
    .await
    .unwrap()
    .unwrap();
    let code2 = verification.value;

    let (status, headers, second_user) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "phone": phone.clone(),
            "code": code2,
        },
        response_type: crate::models::user::User
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert_eq!(first_user.pk, second_user.pk);
    assert_eq!(first_user.username, second_user.username);
}

#[tokio::test]
async fn test_phone_login_with_invalid_code() {
    let TestContextV3 { app, now, ddb, .. } = setup_v3().await;

    let phone = format!("+1555{:07}", now % 10000000 + 200);

    // Send verification code
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/auth/verification/send-verification-code",
        body: {
            "phone": phone.clone(),
        },
        response_type: super::verification::send_code::SendCodeResponse
    };
    assert_eq!(status, 200);

    // Attempt login with invalid code
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "phone": phone.clone(),
            "code": "999999",
        }
    };

    assert_eq!(status, 400);

    // Verify attempt count was incremented
    let verification = crate::models::phone::PhoneVerification::get(
        &ddb,
        Partition::Phone(phone.clone()),
        Some(EntityType::PhoneVerification),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(verification.attempt_count, 1);
}

#[tokio::test]
async fn test_phone_login_without_verification_code() {
    let TestContextV3 { app, now, .. } = setup_v3().await;

    let phone = format!("+1555{:07}", now % 10000000 + 300);

    // Attempt login without sending verification code first
    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/auth/login",
        body: {
            "phone": phone.clone(),
            "code": "123456",
        }
    };

    assert_eq!(status, 400);
}
