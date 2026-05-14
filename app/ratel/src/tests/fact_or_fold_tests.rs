//! Integration tests for *Fact or Fold* admin headline + settings endpoints
//! (PR1 surface). Round/lobby flows are covered in PR3+.
//!
//! Verifies:
//!  - All admin endpoints reject non-admin sessions (roadmap §FR-39)
//!  - Headline CRUD happy path
//!  - Validation invariants: body length, difficulty range, past schedule,
//!    required string fields
//!  - `Live` headlines lock all field edits and deletes (§FR-43)
//!  - `publish` with `scheduled_at = None` moves the headline to `Live`
//!  - Settings read returns defaults; PUT persists; out-of-range rejected.
//!
//! Note: Dioxus server functions wrap each named body parameter as a JSON
//! key. Our controllers use `req: ...` for bodies, so test bodies are wrapped
//! under the `req` key.

use super::*;

use crate::features::fact_or_fold::types::{
    FactOrFoldSettingsResponse, HeadlineResponse, HeadlineStatus,
};

// ── Helpers ───────────────────────────────────────────────────────

/// A body excerpt at the minimum length (200 chars).
fn valid_body() -> String {
    "x".repeat(200)
}

async fn create_headline(ctx: &TestContext, admin: &axum::http::HeaderMap) -> HeadlineResponse {
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/headlines",
        headers: admin.clone(),
        body: {
            "req": {
                "headline_text": "Headline",
                "body_excerpt": valid_body(),
                "verdict": "REAL",
                "difficulty": 3,
                "category_tags": ["경제"],
                "source_label": "Korea Times",
                "insider_statement": "The insider knows.",
                "reveal_summary": "Confirmed by source.",
                "reveal_sources": [],
                "scheduled_at": null,
            }
        },
        response_type: HeadlineResponse,
    };
    assert_eq!(status, 200, "create_headline expected 200, got {}", status);
    body
}

// ── Auth gating (FR-39) ──────────────────────────────────────────

#[tokio::test]
async fn test_create_headline_rejects_non_admin() {
    let ctx = TestContext::setup().await;
    // ctx.test_user is Individual, not Admin
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/headlines",
        headers: ctx.test_user.1.clone(),
        body: {
            "req": {
                "headline_text": "x",
                "body_excerpt": valid_body(),
                "verdict": "REAL",
                "difficulty": 1,
                "source_label": "s",
                "insider_statement": "i",
                "reveal_summary": "r"
            }
        }
    };
    assert_ne!(status, 200, "non-admin must be rejected from create");
}

#[tokio::test]
async fn test_create_headline_rejects_unauthenticated() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/headlines",
        body: {
            "req": {
                "headline_text": "x",
                "body_excerpt": valid_body(),
                "verdict": "REAL",
                "difficulty": 1,
                "source_label": "s",
                "insider_statement": "i",
                "reveal_summary": "r"
            }
        }
    };
    assert_ne!(status, 200, "unauthenticated request must be rejected");
}

#[tokio::test]
async fn test_list_headlines_rejects_non_admin() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/headlines",
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200, "non-admin must be rejected from list");
}

#[tokio::test]
async fn test_get_settings_rejects_non_admin() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/settings",
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200, "non-admin must be rejected from settings GET");
}

// ── Headline CRUD ────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_headline() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;

    let created = create_headline(&ctx, &admin_headers).await;
    assert!(
        matches!(created.status, HeadlineStatus::Draft),
        "newly created headline must be Draft, got {:?}",
        created.status
    );
    let headline_id = created.id.0.clone();

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/headlines/{}", headline_id),
        headers: admin_headers,
        response_type: HeadlineResponse,
    };
    assert_eq!(status, 200, "get_headline expected 200");
    let fetched = body;
    assert_eq!(fetched.id.0, headline_id);
    assert_eq!(fetched.headline_text, "Headline");
}

#[tokio::test]
async fn test_list_headlines_returns_created() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;

    let created = create_headline(&ctx, &admin_headers).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/headlines",
        headers: admin_headers,
        response_type: crate::common::types::ListResponse<HeadlineResponse>,
    };
    assert_eq!(status, 200, "list expected 200");
    let list = body;
    assert!(
        list.items.iter().any(|h| h.id.0 == created.id.0),
        "list should contain the headline we just created"
    );
}

#[tokio::test]
async fn test_update_headline_patches_fields() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let created = create_headline(&ctx, &admin_headers).await;
    let headline_id = created.id.0;

    let (status, _, body) = crate::test_patch! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/admin/headlines/{}", headline_id),
        headers: admin_headers,
        body: {
            "req": { "headline_text": "Patched", "difficulty": 4 }
        },
        response_type: HeadlineResponse,
    };
    assert_eq!(status, 200, "patch expected 200");
    let updated = body;
    assert_eq!(updated.headline_text, "Patched");
    assert_eq!(updated.difficulty, 4);
    // Untouched field stays:
    assert_eq!(updated.body_excerpt.chars().count(), 200);
}

#[tokio::test]
async fn test_delete_headline_soft_deletes() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let created = create_headline(&ctx, &admin_headers).await;
    let headline_id = created.id.0;

    let (status, _, body) = crate::test_delete! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/admin/headlines/{}", headline_id),
        headers: admin_headers,
        response_type: HeadlineResponse,
    };
    assert_eq!(status, 200, "delete expected 200");
    let deleted = body;
    assert!(
        matches!(deleted.status, HeadlineStatus::Deleted),
        "deleted headline must report Deleted status, got {:?}",
        deleted.status
    );
}

// ── Validation invariants (FR-40, FR-42) ─────────────────────────

#[tokio::test]
async fn test_create_rejects_body_too_short() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/headlines",
        headers: admin_headers,
        body: {
            "req": {
                "headline_text": "Headline",
                "body_excerpt": "too short",
                "verdict": "REAL",
                "difficulty": 3,
                "source_label": "s",
                "insider_statement": "i",
                "reveal_summary": "r",
            }
        }
    };
    assert_ne!(status, 200, "body shorter than 200 chars must be rejected");
}

#[tokio::test]
async fn test_create_rejects_invalid_difficulty() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/headlines",
        headers: admin_headers,
        body: {
            "req": {
                "headline_text": "Headline",
                "body_excerpt": valid_body(),
                "verdict": "REAL",
                "difficulty": 9,
                "source_label": "s",
                "insider_statement": "i",
                "reveal_summary": "r",
            }
        }
    };
    assert_ne!(status, 200, "difficulty outside 1..=5 must be rejected");
}

#[tokio::test]
async fn test_create_rejects_empty_insider_statement() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/headlines",
        headers: admin_headers,
        body: {
            "req": {
                "headline_text": "Headline",
                "body_excerpt": valid_body(),
                "verdict": "REAL",
                "difficulty": 3,
                "source_label": "Korea Times",
                "insider_statement": "   ",
                "reveal_summary": "r",
            }
        }
    };
    assert_ne!(status, 200, "blank insider_statement must be rejected");
}

// ── Publish + lock (FR-42, FR-43) ────────────────────────────────

#[tokio::test]
async fn test_publish_now_moves_to_live() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let created = create_headline(&ctx, &admin_headers).await;
    let headline_id = created.id.0;

    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/admin/headlines/{}/publish", headline_id),
        headers: admin_headers,
        body: { "req": { "scheduled_at": null } },
        response_type: HeadlineResponse,
    };
    assert_eq!(status, 200, "publish-now expected 200, got {}", status);
    let live = body;
    assert!(
        matches!(live.status, HeadlineStatus::Live),
        "publish-now must transition to Live, got {:?}",
        live.status
    );
    assert!(live.scheduled_at.is_none(), "scheduled_at must be cleared");
}

#[tokio::test]
async fn test_publish_in_the_past_rejected() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let created = create_headline(&ctx, &admin_headers).await;
    let headline_id = created.id.0;

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/admin/headlines/{}/publish", headline_id),
        headers: admin_headers,
        body: { "req": { "scheduled_at": 1_000_000_i64 } }
    };
    assert_ne!(status, 200, "past scheduled_at must be rejected");
}

#[tokio::test]
async fn test_update_locked_when_live() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    let created = create_headline(&ctx, &admin_headers).await;
    let headline_id = created.id.0;

    // Publish to Live
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/headlines/{}/publish", headline_id),
        headers: admin_headers.clone(),
        body: { "req": { "scheduled_at": null } }
    };
    assert_eq!(status, 200, "publish step must succeed");

    // Edit attempt on a Live headline must be rejected
    let (status, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/headlines/{}", headline_id),
        headers: admin_headers.clone(),
        body: { "req": { "headline_text": "tampered" } }
    };
    assert_ne!(status, 200, "Live headline must reject field edits");

    // Delete attempt also rejected
    let (status, _, _) = crate::test_delete! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/admin/headlines/{}", headline_id),
        headers: admin_headers,
    };
    assert_ne!(status, 200, "Live headline must reject delete");
}

// ── Settings ─────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_settings_returns_defaults() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/settings",
        headers: admin_headers,
        response_type: FactOrFoldSettingsResponse,
    };
    assert_eq!(status, 200, "get_settings expected 200");
    let settings = body;
    let defaults = FactOrFoldSettingsResponse::default();
    assert_eq!(settings.round_capacity, defaults.round_capacity);
    assert_eq!(settings.min_bet_rp, defaults.min_bet_rp);
}

#[tokio::test]
async fn test_update_settings_persists_patch() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;

    let (status, _, body) = crate::test_put! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/settings",
        headers: admin_headers.clone(),
        body: { "req": { "round_capacity": 6, "min_bet_rp": 200 } },
        response_type: FactOrFoldSettingsResponse,
    };
    assert_eq!(status, 200, "put_settings expected 200");
    let updated = body;
    assert_eq!(updated.round_capacity, 6);
    assert_eq!(updated.min_bet_rp, 200);

    // Re-read to confirm persistence
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/settings",
        headers: admin_headers,
        response_type: FactOrFoldSettingsResponse,
    };
    assert_eq!(status, 200);
    let reread = body;
    assert_eq!(reread.round_capacity, 6);
    assert_eq!(reread.min_bet_rp, 200);
}

#[tokio::test]
async fn test_update_settings_rejects_out_of_range() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;
    // round_capacity must be >= 2; 1 should be rejected
    let (status, _, _) = crate::test_put! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/settings",
        headers: admin_headers,
        body: { "req": { "round_capacity": 1 } }
    };
    assert_ne!(status, 200, "round_capacity=1 must be rejected");
}
