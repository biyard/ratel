use super::*;

// Helper: sign up a brand-new passwordless email user via the public endpoints.
// Returns the email used.
async fn signup_email_user(app: &axum::Router) -> String {
    let email = format!("auth-{}@test.com", uuid::Uuid::now_v7());
    // validate_username caps at 20 chars; uuid.simple() is 32 hex, so take
    // the trailing 12 (random) chars → "u" + 12 = 13 chars, still unique.
    let username = format!("u{}", &uuid::Uuid::now_v7().simple().to_string()[20..]);

    let (s, _, _) = crate::test_post! {
        app: app.clone(),
        path: "/api/auth/verification/send-verification-code",
        body: { "req": { "email": email } }
    };
    assert_eq!(s, 200, "send-code should succeed");

    let (s, _, _) = crate::test_post! {
        app: app.clone(),
        path: "/api/auth/signup",
        body: { "req": {
            "email": email, "code": "000000", "display_name": "T",
            "username": username, "profile_url": "", "description": "",
            "term_agreed": true, "informed_agreed": false
        } }
    };
    assert_eq!(s, 200, "signup should succeed");

    email
}

#[tokio::test]
async fn test_email_code_login_existing_user() {
    let ctx = TestContext::setup().await;

    let email = signup_email_user(&ctx.app).await;

    // login with email + code
    let (s, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/auth/login",
        body: { "req": { "email": email, "code": "000000" } }
    };
    assert_eq!(s, 200, "email-code login: {:?}", body);
}

#[tokio::test]
async fn test_email_code_login_unknown_user_returns_not_found() {
    let ctx = TestContext::setup().await;

    let email = format!("nouser-{}@test.com", uuid::Uuid::now_v7());
    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/auth/verification/send-verification-code",
        body: { "req": { "email": email } }
    };
    let (s, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/auth/login",
        body: { "req": { "email": email, "code": "000000" } }
    };
    assert_ne!(
        s, 200,
        "unknown user must not log in (frontend branches to signup)"
    );
}

#[tokio::test]
async fn test_email_code_signup_no_password_creates_user() {
    let ctx = TestContext::setup().await;

    let email = format!("signup-{}@test.com", uuid::Uuid::now_v7());
    // validate_username caps at 20 chars; uuid.simple() is 32 hex, so take
    // the trailing 12 (random) chars → "u" + 12 = 13 chars, still unique.
    let username = format!("u{}", &uuid::Uuid::now_v7().simple().to_string()[20..]);

    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/auth/verification/send-verification-code",
        body: { "req": { "email": email } }
    };
    let (s, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/auth/signup",
        body: { "req": {
            "email": email, "code": "000000", "display_name": "T",
            "username": username, "profile_url": "", "description": "",
            "term_agreed": true, "informed_agreed": false
        } }
    };
    assert_eq!(s, 200, "passwordless signup: {:?}", body);
    // No `password` field is sent; user is created and session established.
}

/// Backward compatibility / no-migration proof: an account that already has a
/// stored password (i.e. created before this change) must still log in via the
/// new email+code flow, with the password ignored. `find_by_email` keys off the
/// email index only, so the legacy `password` attribute is irrelevant.
#[tokio::test]
async fn test_legacy_password_user_logs_in_with_email_code() {
    let ctx = TestContext::setup().await;

    let email = format!("legacy-{}@test.com", uuid::Uuid::now_v7());
    // validate_username caps at 20 chars; uuid.simple() is 32 hex, so take
    // the trailing 12 (random) chars → "u" + 12 = 13 chars, still unique.
    let username = format!("u{}", &uuid::Uuid::now_v7().simple().to_string()[20..]);

    // Simulate a pre-existing account the same way old signups created them:
    // a real User row carrying a `Some(password_hash)`.
    let legacy = crate::common::models::auth::User::new(
        "Legacy User".to_string(),
        email.clone(),
        String::new(),
        true,
        true,
        crate::common::types::UserType::Individual,
        username,
        Some("legacy-sha3-hash".to_string()),
    );
    legacy.create(&ctx.ddb).await.expect("create legacy user");

    // Request a fresh code (no pre-existing verification record needed) ...
    let (s, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/auth/verification/send-verification-code",
        body: { "req": { "email": email } }
    };
    assert_eq!(s, 200, "send-code for legacy email should succeed");

    // ... and log in with email + code only — no password supplied.
    let (s, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/auth/login",
        body: { "req": { "email": email, "code": "000000" } }
    };
    assert_eq!(
        s, 200,
        "legacy password-having user must log in via email-code: {:?}",
        body
    );
}
