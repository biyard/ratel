//! Server-function integration tests for the cross-posting feature.
//!
//! Coverage matrix per `roadmap/cross-posting.md` § Phase 1A:
//!
//! | Endpoint                             | Unauth | Owner happy-path | Cross-user / not-allowed |
//! |--------------------------------------|--------|------------------|--------------------------|
//! | POST  `/connections/bluesky/connect` |   ✅   |     skipped*     |             —            |
//! | GET   `/connections`                 |   ✅   |        ✅        |             —            |
//! | PATCH `/connections/{platform}`      |   ✅   |        ✅        |             —            |
//! | DELETE `/connections/{platform}`     |   ✅   |        ✅        |             —            |
//! | GET   `/posts/{id}/syndication`      |   ✅   |        ✅        |             ✅           |
//! | POST  `/posts/{id}/jobs/{p}/retry`   |   ✅   |        ✅        |             ✅           |
//!
//! \* The `connect_bluesky` happy path is skipped because the handler calls
//!    `BlueskyAdapter::create_session` against the real Bluesky API. Stubbing
//!    that adapter is tracked separately (`BYPASS_PLATFORM_API=mock`,
//!    design doc § Test plan); for now the tests only exercise the
//!    pre-adapter validation (auth + empty-input rejection).

use super::*;
use crate::common::types::*;
use crate::features::cross_posting::models::{
    ConnectionStatus, JobState, SocialConnection, SyndicationJob,
};
use crate::features::cross_posting::types::SocialPlatform;

// ─────────────────────────────────────────────────────────────────────────────
// connect_bluesky — POST /api/cross-posting/connections/bluesky/connect
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_connect_bluesky_unauthenticated() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _, _) = crate::test_post! {
        app: app,
        path: "/api/cross-posting/connections/bluesky/connect",
        body: { "handle": "alice.bsky.social", "app_password": "abcd-efgh-ijkl-mnop" }
    };
    assert_ne!(
        status, 200,
        "unauthenticated connect_bluesky must not succeed"
    );
}

#[tokio::test]
async fn test_connect_bluesky_empty_input_rejected() {
    let TestContext { app, test_user, .. } = TestContext::setup().await;

    // Empty handle / app_password short-circuits before the Bluesky API call,
    // so this is safe to run without the platform mock.
    let (status, _, body) = crate::test_post! {
        app: app,
        path: "/api/cross-posting/connections/bluesky/connect",
        headers: test_user.1.clone(),
        body: { "handle": "", "app_password": "" }
    };
    assert_ne!(
        status, 200,
        "empty handle/password must be rejected: {:?}",
        body
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// list_connections — GET /api/cross-posting/connections
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_list_connections_unauthenticated() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _, _) = crate::test_get! {
        app: app,
        path: "/api/cross-posting/connections",
    };
    assert_ne!(status, 200, "unauthenticated list_connections must fail");
}

#[tokio::test]
async fn test_list_connections_empty_for_new_user() {
    let TestContext { app, test_user, .. } = TestContext::setup().await;

    let (status, _, body) = crate::test_get! {
        app: app,
        path: "/api/cross-posting/connections",
        headers: test_user.1.clone(),
    };
    assert_eq!(status, 200, "list_connections: {:?}", body);
    let arr = body.as_array().expect("response is JSON array");
    assert!(
        arr.is_empty(),
        "new user has no connections; got {}: {:?}",
        arr.len(),
        body
    );
}

#[tokio::test]
async fn test_list_connections_returns_inserted_row() {
    let ctx = TestContext::setup().await;

    seed_bluesky_connection(&ctx, ConnectionStatus::Connected, true).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/cross-posting/connections",
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "list response: {:?}", body);
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 1, "one connection expected: {:?}", body);
    assert_eq!(arr[0]["platform"], "bluesky");
    assert_eq!(arr[0]["status"], "connected");
    assert_eq!(arr[0]["auto_post_enabled"], true);
}

// ─────────────────────────────────────────────────────────────────────────────
// toggle_auto_post — PATCH /api/cross-posting/connections/{platform}
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_toggle_auto_post_unauthenticated() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _, _) = crate::test_patch! {
        app: app,
        path: "/api/cross-posting/connections/bluesky",
        body: { "auto_post_enabled": false }
    };
    assert_ne!(status, 200, "unauthenticated toggle must not succeed");
}

#[tokio::test]
async fn test_toggle_auto_post_flips_value() {
    let ctx = TestContext::setup().await;
    seed_bluesky_connection(&ctx, ConnectionStatus::Connected, true).await;

    // Flip to false. Dioxus server functions wrap the body in `{ "req": ... }`
    // (same pattern setup.rs uses for login) — bare `{ "auto_post_enabled" }`
    // fails deserialization with `missing field `req``.
    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: "/api/cross-posting/connections/bluesky",
        headers: ctx.test_user.1.clone(),
        body: { "req": { "auto_post_enabled": false } }
    };
    assert_eq!(status, 200, "toggle off: {:?}", body);

    // Re-list and confirm the persisted change.
    let (_, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/cross-posting/connections",
        headers: ctx.test_user.1.clone(),
    };
    let arr = body.as_array().unwrap();
    assert_eq!(arr[0]["auto_post_enabled"], false, "value flipped: {:?}", body);
}

// ─────────────────────────────────────────────────────────────────────────────
// disconnect — DELETE /api/cross-posting/connections/{platform}
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_disconnect_unauthenticated() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _, _) = crate::test_delete! {
        app: app,
        path: "/api/cross-posting/connections/bluesky",
    };
    assert_ne!(status, 200, "unauthenticated disconnect must fail");
}

#[tokio::test]
async fn test_disconnect_marks_revoked() {
    let ctx = TestContext::setup().await;
    seed_bluesky_connection(&ctx, ConnectionStatus::Connected, true).await;

    let (status, _, body) = crate::test_delete! {
        app: ctx.app.clone(),
        path: "/api/cross-posting/connections/bluesky",
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "disconnect: {:?}", body);

    // Soft-delete: the row stays in the listing but flips to `Revoked` and
    // its credential ciphertext is zeroed (FR-1 #7).
    let (_, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/cross-posting/connections",
        headers: ctx.test_user.1.clone(),
    };
    let arr = body.as_array().unwrap();
    assert_eq!(arr.len(), 1, "row stays for history: {:?}", body);
    assert_eq!(arr[0]["status"], "revoked", "status flipped: {:?}", body);
}

// ─────────────────────────────────────────────────────────────────────────────
// get_syndication_panel — GET /api/cross-posting/posts/{id}/syndication
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_syndication_panel_unauthenticated() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _, _) = crate::test_get! {
        app: app,
        path: "/api/cross-posting/posts/nonexistent/syndication",
    };
    assert_ne!(status, 200, "unauthenticated request must fail");
}

#[tokio::test]
async fn test_get_syndication_panel_other_users_post_rejected() {
    let ctx = TestContext::setup().await;
    let post_id = create_draft_post(&ctx, &ctx.test_user.1).await;

    // A second user requesting the first user's panel must hit NotAuthorized.
    let (_, headers2) = ctx.create_another_user().await;
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/cross-posting/posts/{}/syndication", post_id),
        headers: headers2,
    };
    assert_ne!(
        status, 200,
        "non-author must not read syndication panel: {:?}",
        body
    );
}

#[tokio::test]
async fn test_get_syndication_panel_owner_empty_jobs() {
    let ctx = TestContext::setup().await;
    let post_id = create_draft_post(&ctx, &ctx.test_user.1).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/cross-posting/posts/{}/syndication", post_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "panel: {:?}", body);
    let jobs = body["jobs"].as_array().expect("jobs array");
    assert!(jobs.is_empty(), "fresh draft has no syndication jobs yet");
}

#[tokio::test]
async fn test_get_syndication_panel_returns_seeded_jobs() {
    let ctx = TestContext::setup().await;
    let post_id = create_draft_post(&ctx, &ctx.test_user.1).await;
    seed_syndication_job(&ctx, &post_id, SocialPlatform::Bluesky, JobState::Published).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/cross-posting/posts/{}/syndication", post_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "panel: {:?}", body);
    let jobs = body["jobs"].as_array().expect("jobs array");
    assert_eq!(jobs.len(), 1, "one seeded job: {:?}", body);
    assert_eq!(jobs[0]["platform"], "bluesky");
    assert_eq!(jobs[0]["state"], "published");
}

// ─────────────────────────────────────────────────────────────────────────────
// retry_job — POST /api/cross-posting/posts/{id}/jobs/{platform}/retry
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_retry_job_unauthenticated() {
    let TestContext { app, .. } = TestContext::setup().await;

    let (status, _, _) = crate::test_post! {
        app: app,
        path: "/api/cross-posting/posts/nonexistent/jobs/bluesky/retry",
    };
    assert_ne!(status, 200, "unauthenticated retry must fail");
}

#[tokio::test]
async fn test_retry_job_other_user_rejected() {
    let ctx = TestContext::setup().await;
    let post_id = create_draft_post(&ctx, &ctx.test_user.1).await;
    seed_syndication_job(&ctx, &post_id, SocialPlatform::Bluesky, JobState::Failed).await;

    let (_, headers2) = ctx.create_another_user().await;
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/cross-posting/posts/{}/jobs/bluesky/retry", post_id),
        headers: headers2,
    };
    assert_ne!(
        status, 200,
        "non-author must not retry someone else's job: {:?}",
        body
    );
}

#[tokio::test]
async fn test_retry_job_pending_state_allowed() {
    let ctx = TestContext::setup().await;
    let post_id = create_draft_post(&ctx, &ctx.test_user.1).await;
    seed_syndication_job(&ctx, &post_id, SocialPlatform::Bluesky, JobState::Pending).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/cross-posting/posts/{}/jobs/bluesky/retry", post_id),
        headers: ctx.test_user.1.clone(),
    };
    // Per `retry_job_handler`: only `Published` is blocked. `Pending` rows
    // (e.g. dispatcher Lambda crashed mid-flight, or EventBridge dropped
    // the event) MUST allow user-initiated retry so the author isn't
    // dependent on infra recovery.
    assert_eq!(status, 200, "retry on Pending must succeed: {:?}", body);
}

#[tokio::test]
async fn test_retry_job_failed_flips_to_pending() {
    let ctx = TestContext::setup().await;
    let post_id = create_draft_post(&ctx, &ctx.test_user.1).await;
    seed_syndication_job(&ctx, &post_id, SocialPlatform::Bluesky, JobState::Failed).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/cross-posting/posts/{}/jobs/bluesky/retry", post_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "retry: {:?}", body);

    // Re-read via the panel and confirm the row is now Pending. Stage 2's
    // EventBridge Pipe fires off the MODIFY event from this transition.
    let (_, _, panel) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/cross-posting/posts/{}/syndication", post_id),
        headers: ctx.test_user.1.clone(),
    };
    let jobs = panel["jobs"].as_array().expect("jobs array");
    assert_eq!(jobs[0]["state"], "pending", "state after retry: {:?}", panel);
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Seed a `SocialConnection` row directly in DynamoDB. The real connect
/// flow goes through Bluesky's `createSession` which we don't mock yet,
/// so the listing / toggle / disconnect tests bypass the API and write
/// the row themselves to exercise the read-side logic.
async fn seed_bluesky_connection(
    ctx: &TestContext,
    status: ConnectionStatus,
    auto_post: bool,
) {
    let user_pk = ctx.test_user.0.pk.clone();
    let now = crate::common::utils::time::now();
    let conn = SocialConnection {
        pk: user_pk,
        sk: EntityType::SocialConnection(SocialPlatform::Bluesky.to_string()),
        platform_status: format!("bluesky#{}", status),
        platform: SocialPlatform::Bluesky,
        status,
        external_handle: "alice.bsky.social".to_string(),
        external_user_id: "did:plc:alice".to_string(),
        // Sentinel ciphertext — these tests never decrypt; if you add a
        // decrypt path, replace with `aead::seal(...)` against
        // `CROSS_POSTING_DATA_KEY`.
        credential_ciphertext: vec![1, 2, 3, 4],
        token_expires_at: None,
        auto_post_enabled: auto_post,
        posts_syndicated_count: 0,
        last_synced_at: None,
        created_at: now,
        updated_at: now,
    };
    conn.create(&ctx.ddb).await.expect("seed connection");
}

/// Seed a `SyndicationJob` row directly in DynamoDB for panel / retry
/// tests. The fan-out Lambda would normally write these in Stage 1.
async fn seed_syndication_job(
    ctx: &TestContext,
    post_id: &str,
    platform: SocialPlatform,
    state: JobState,
) {
    let now = crate::common::utils::time::now();
    let job = SyndicationJob {
        pk: Partition::Feed(post_id.to_string()),
        sk: EntityType::SyndicationJob(platform.to_string()),
        dispatch_shard: None,
        engagement_shard: None,
        next_attempt_at: 0,
        engagement_next_at: 0,
        author_user_id: ctx.test_user.0.pk.clone(),
        platform,
        state,
        attempts: 0,
        last_error_category: None,
        last_error_message: None,
        external_post_id: None,
        external_post_url: None,
        body_snapshot_len: 0,
        backlink_url: format!("https://ratel.foundation/posts/{}?utm_source={}", post_id, platform),
        dispatch_lock_id: None,
        lock_acquired_at: None,
        created_at: now,
        updated_at: now,
    };
    job.create(&ctx.ddb).await.expect("seed job");
}

/// POST `/api/posts` (creates an empty draft owned by `headers`'s user)
/// and return the inner post id (without the `POST#` prefix).
async fn create_draft_post(ctx: &TestContext, headers: &axum::http::HeaderMap) -> String {
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/posts",
        headers: headers.clone(),
    };
    assert_eq!(status, 200, "create draft post: {:?}", body);
    let pk = body["post_pk"]
        .as_str()
        .expect("post_pk in response");
    // Strip the `FEED#` prefix (Partition::Feed display) to match the
    // SubPartition (FeedPartition) form that cross-posting endpoints accept
    // in their path params — `FeedPartition` serializes / parses without
    // the prefix.
    pk.strip_prefix("FEED#").unwrap_or(pk).to_string()
}
