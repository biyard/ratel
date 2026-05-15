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

use crate::common::types::Partition;
use crate::features::arcade::games::fact_or_fold::models::FactFoldRound;
use crate::features::arcade::models::ArcadeWalletBalance;
use crate::features::arcade::games::fact_or_fold::controllers::settlement::SettleRoundResponse;
use crate::features::arcade::games::fact_or_fold::types::{
    BetResponse, BetSide, FactOrFoldSettingsResponse, HeadlineResponse, HeadlineStatus,
    InsiderStatementResponse, ListBetsResponse, ListParticipantsResponse, ListRationalesResponse,
    LobbyResponse, ParticipantResponse, QueueAlarmResponse, RationaleResponse,
    RoundHeadlineResponse, RoundResponse, RoundStatus, Verdict,
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

// ── Queue alarm (FR-45) ──────────────────────────────────────────

#[tokio::test]
async fn test_queue_alarm_rejects_non_admin() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/queue/alarm",
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200, "non-admin must be rejected from queue alarm");
}

#[tokio::test]
async fn test_queue_alarm_alerts_when_empty_or_near_threshold() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;

    // Empty queue → alert=true, days_remaining=0
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/queue/alarm",
        headers: admin_headers.clone(),
        response_type: QueueAlarmResponse,
    };
    assert_eq!(status, 200);
    assert!(body.alert, "empty queue must trigger alert");
    assert_eq!(body.queue_days_remaining, 0.0);
    assert_eq!(body.scheduled_future_count, 0);

    // Schedule a headline 1 day out (well inside the 5-day default threshold)
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let one_day_out = now + 86_400_000;
    let _ = create_headline_scheduled(&ctx, &admin_headers, one_day_out).await;

    let (_, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/queue/alarm",
        headers: admin_headers.clone(),
        response_type: QueueAlarmResponse,
    };
    assert!(body.alert, "1-day out queue must still trigger alert");
    assert!(body.queue_days_remaining > 0.0 && body.queue_days_remaining < 2.0);
    assert_eq!(body.scheduled_future_count, 1);
}

#[tokio::test]
async fn test_queue_alarm_clears_when_far_future_scheduled() {
    let ctx = TestContext::setup().await;
    let (_, admin_headers) = ctx.create_admin_user().await;

    // Schedule far enough out (10 days) to clear the default 5-day alert.
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let ten_days_out = now + 10 * 86_400_000;
    let _ = create_headline_scheduled(&ctx, &admin_headers, ten_days_out).await;

    let (_, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/admin/queue/alarm",
        headers: admin_headers,
        response_type: QueueAlarmResponse,
    };
    assert!(!body.alert, "far-future queue must clear the alert");
    assert!(body.queue_days_remaining > 5.0);
}

/// Like `create_headline` but with a non-null `scheduled_at` so the
/// resulting row lands in `Scheduled` status.
async fn create_headline_scheduled(
    ctx: &TestContext,
    admin: &axum::http::HeaderMap,
    scheduled_at: i64,
) -> HeadlineResponse {
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
                "scheduled_at": scheduled_at,
            }
        },
        response_type: HeadlineResponse,
    };
    assert_eq!(status, 200, "create_headline_scheduled expected 200");
    body
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

// ── Lobby + matching (PR3) ───────────────────────────────────────

/// Top up a user's chip balance directly via DDB so the lobby's
/// chip gate + buy_in pass without needing an RP→chip convert
/// round-trip. Use this for any test that exercises `join_lobby`.
async fn grant_chips_for_test(ctx: &TestContext, user_pk: &Partition, chips: i64) {
    let user_id = user_pk
        .to_string()
        .strip_prefix("USER#")
        .unwrap_or(&user_pk.to_string())
        .to_string();
    let (pk, sk) = ArcadeWalletBalance::keys(&user_id);
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let row = ArcadeWalletBalance {
        pk,
        sk,
        created_at: now,
        updated_at: now,
        chip_balance: chips,
    };
    row.upsert(&ctx.ddb).await.expect("grant chips upsert");
}

/// Drop the join balance gate for tests so we can exercise the
/// matching loop without first granting RP / chips to every test
/// user. PR4c moved the gate from RP to chips, so we now zero the
/// arcade-wide `default_buy_in_chips` (the wallet still rejects
/// `chips <= 0` so we use the minimum allowed of 1). The legacy
/// `min_bet_rp` knob is still zeroed because the FOF bet endpoint
/// continues to validate against it.
async fn relax_balance_gate(ctx: &TestContext, admin: &axum::http::HeaderMap) {
    let (status, _, _) = crate::test_put! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/admin/settings",
        headers: admin.clone(),
        body: { "req": { "min_bet_rp": 0 } }
    };
    assert_eq!(status, 200, "relax_balance_gate (FOF settings) must succeed");

    let (status, _, _) = crate::test_put! {
        app: ctx.app.clone(),
        path: "/api/arcade/admin/settings",
        headers: admin.clone(),
        body: { "req": { "default_buy_in_chips": 1 } }
    };
    assert_eq!(
        status, 200,
        "relax_balance_gate (arcade settings) must succeed"
    );
}

/// Create a Draft headline, then publish-now → Live so it's a
/// matching candidate for the lobby.
async fn create_live_headline(ctx: &TestContext, admin: &axum::http::HeaderMap) -> HeadlineResponse {
    let h = create_headline(ctx, admin).await;
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/headlines/{}/publish", h.id.0),
        headers: admin.clone(),
        body: { "req": { "scheduled_at": null } },
        response_type: HeadlineResponse,
    };
    assert_eq!(status, 200, "publish-now must succeed: {:?}", body);
    body
}

#[tokio::test]
async fn test_get_lobby_rejects_unauthenticated() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/lobby",
    };
    assert_ne!(status, 200, "unauth must be rejected from lobby read");
}

#[tokio::test]
async fn test_lobby_no_headline_reports_unavailable() {
    let ctx = TestContext::setup().await;
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/lobby",
        headers: ctx.test_user.1.clone(),
        response_type: LobbyResponse,
    };
    assert_eq!(status, 200);
    assert!(!body.headline_available, "no headlines yet");
    assert!(body.current_round.is_none(), "no waiting round yet");
    assert!(!body.can_join, "can't join with nothing in the queue");
}

#[tokio::test]
async fn test_join_no_headline_returns_unavailable() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    relax_balance_gate(&ctx, &admin).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
    };
    // Service-unavailable status shape — we only assert non-200.
    assert_ne!(status, 200, "join without a headline must fail");
}

#[tokio::test]
async fn test_join_creates_round_and_lobby_state_reflects_it() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    relax_balance_gate(&ctx, &admin).await;
    let _ = create_live_headline(&ctx, &admin).await;
    grant_chips_for_test(&ctx, &ctx.test_user.0.pk, 1_000).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
        response_type: RoundResponse,
    };
    assert_eq!(status, 200, "join must succeed once a headline exists");
    let round = body;
    assert_eq!(round.participant_pks.len(), 1, "first joiner is alone");
    assert!(matches!(round.status, RoundStatus::Waiting), "still waiting for more players, got {:?}", round.status);

    // Lobby reads back the same round with already_joined = true.
    let (_, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/lobby",
        headers: ctx.test_user.1.clone(),
        response_type: LobbyResponse,
    };
    assert!(body.current_round.is_some());
    assert!(body.already_joined);
    assert!(!body.can_join, "already joined → can't join again");
}

#[tokio::test]
async fn test_join_again_rejected() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    relax_balance_gate(&ctx, &admin).await;
    let _ = create_live_headline(&ctx, &admin).await;
    grant_chips_for_test(&ctx, &ctx.test_user.0.pk, 1_000).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200);

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200, "second join by same user must fail");
}

#[tokio::test]
async fn test_get_round_after_join() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    relax_balance_gate(&ctx, &admin).await;
    let _ = create_live_headline(&ctx, &admin).await;
    grant_chips_for_test(&ctx, &ctx.test_user.0.pk, 1_000).await;

    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
        response_type: RoundResponse,
    };
    let round_id = body.id.0;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}", round_id),
        headers: ctx.test_user.1.clone(),
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.id.0, round_id);
    assert_eq!(body.participant_pks.len(), 1);
}

#[tokio::test]
async fn test_leave_after_join_clears_participant() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    relax_balance_gate(&ctx, &admin).await;
    let _ = create_live_headline(&ctx, &admin).await;
    grant_chips_for_test(&ctx, &ctx.test_user.0.pk, 1_000).await;

    let (_, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
        response_type: RoundResponse,
    };

    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: "/api/fact-or-fold/lobby/leave",
        headers: ctx.test_user.1.clone(),
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(body.participant_pks.is_empty());
}

#[tokio::test]
async fn test_get_round_not_found() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/rounds/nonexistent-id",
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200);
}

// ── Round play (PR4) ─────────────────────────────────────────────

/// Drive the round all the way to "stage 2 (Bet)" by joining the
/// caller + 3 fresh users, which fills capacity=4 and auto-starts.
/// Returns (round_id, headers_for_user_in_round).
#[allow(clippy::type_complexity)]
async fn fill_round_to_capacity(
    ctx: &TestContext,
    admin: &axum::http::HeaderMap,
) -> (String, axum::http::HeaderMap) {
    relax_balance_gate(ctx, admin).await;
    let _ = create_live_headline(ctx, admin).await;

    // First joiner = ctx.test_user
    grant_chips_for_test(ctx, &ctx.test_user.0.pk, 1_000).await;
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
        response_type: RoundResponse,
    };
    assert_eq!(status, 200, "first join must succeed");
    let round_id = body.id.0;

    // Three more fresh users to hit capacity=4
    for _ in 0..3 {
        let (user, headers) = ctx.create_another_user().await;
        grant_chips_for_test(ctx, &user.pk, 1_000).await;
        let (status, _, _) = crate::test_post! {
            app: ctx.app.clone(),
            path: "/api/fact-or-fold/lobby/join",
            headers: headers.clone(),
            response_type: RoundResponse,
        };
        assert_eq!(status, 200, "additional joins must succeed");
    }

    (round_id, ctx.test_user.1.clone())
}

#[tokio::test]
async fn test_round_starts_after_capacity_reached() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(body.status, RoundStatus::NewsReveal), "status was {:?}", body.status);
    assert_eq!(body.participant_pks.len(), 4);
    assert!(body.started_at.is_some());
}

#[tokio::test]
async fn test_bet_rejected_outside_bet_stage() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Round is in NewsReveal — bet must be rejected.
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets", round_id),
        headers: headers,
        body: { "req": { "side": "REAL", "amount_rp": 100 } }
    };
    assert_ne!(status, 200, "bet outside the Bet stage must be rejected");
}

#[tokio::test]
async fn test_heartbeat_updates_last_seen() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/heartbeat", round_id),
        headers: headers,
        response_type: ParticipantResponse,
    };
    assert_eq!(status, 200);
    assert!(body.last_seen_at > 0);
}

#[tokio::test]
async fn test_insider_statement_returns_some_for_one_returns_none_for_others() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    relax_balance_gate(&ctx, &admin).await;
    let _ = create_live_headline(&ctx, &admin).await;
    grant_chips_for_test(&ctx, &ctx.test_user.0.pk, 1_000).await;

    let mut all_user_headers: Vec<axum::http::HeaderMap> =
        vec![ctx.test_user.1.clone()];
    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/fact-or-fold/lobby/join",
        headers: ctx.test_user.1.clone(),
        response_type: RoundResponse,
    };
    let round_id = body.id.0;
    for _ in 0..3 {
        let (user, headers) = ctx.create_another_user().await;
        grant_chips_for_test(&ctx, &user.pk, 1_000).await;
        let (_, _, _) = crate::test_post! {
            app: ctx.app.clone(),
            path: "/api/fact-or-fold/lobby/join",
            headers: headers.clone(),
            response_type: RoundResponse,
        };
        all_user_headers.push(headers);
    }

    // Walk all 4 — exactly one must see the insider statement.
    let mut some_count = 0;
    for headers in all_user_headers {
        let (status, _, body) = crate::test_get! {
            app: ctx.app.clone(),
            path: &format!("/api/fact-or-fold/rounds/{}/insider-statement", round_id),
            headers: headers,
            response_type: InsiderStatementResponse,
        };
        assert_eq!(status, 200);
        if body.statement.is_some() {
            some_count += 1;
        }
    }
    assert_eq!(some_count, 1, "exactly one player should be the insider");
}

#[tokio::test]
async fn test_non_participant_cannot_bet_or_heartbeat() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;

    // A 5th user who never joined.
    let (_, outsider) = ctx.create_another_user().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/rounds/{}/bets", round_id),
        headers: outsider.clone(),
        body: { "req": { "side": "REAL", "amount_rp": 100 } }
    };
    assert_ne!(status, 200, "non-participant must be rejected from bet");

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/heartbeat", round_id),
        headers: outsider,
    };
    assert_ne!(status, 200, "non-participant must be rejected from heartbeat");
}

// ── Stage state machine (PR4 step 3) ─────────────────────────────

/// Reach into DynamoDB and rewind the round's `stage_deadline_at` so
/// the next read/write triggers the lazy-advance ratchet without
/// having to wait wall-clock seconds.
async fn backdate_stage_deadline(
    ctx: &TestContext,
    round_id: &str,
    millis_in_past: i64,
) -> FactFoldRound {
    let (pk, sk) = FactFoldRound::keys(round_id);
    let mut row = FactFoldRound::get(&ctx.ddb, &pk, Some(sk))
        .await
        .expect("ddb read")
        .expect("round must exist");
    let now = crate::common::utils::time::get_now_timestamp_millis();
    row.stage_started_at = Some(now - millis_in_past - 1);
    row.stage_deadline_at = Some(now - millis_in_past);
    row.upsert(&ctx.ddb).await.expect("backdate upsert");
    row
}

#[tokio::test]
async fn test_round_stage_clock_set_on_capacity_reached() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(matches!(body.status, RoundStatus::NewsReveal));
    let started = body.stage_started_at.expect("stage_started_at must be set");
    let deadline = body.stage_deadline_at.expect("stage_deadline_at must be set");
    // 30s NewsReveal default — exact bound is admin-tunable so test
    // for ordering rather than the literal duration.
    assert!(deadline > started, "deadline must be after start");
}

#[tokio::test]
async fn test_get_round_advances_news_reveal_when_deadline_passed() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Backdate so NewsReveal has expired, but Bet (next stage) still
    // has time on the clock. NewsReveal default is 30s → backdating
    // 5s ago lands us inside the Bet window after one advance step.
    backdate_stage_deadline(&ctx, &round_id, 5_000).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(
        matches!(body.status, RoundStatus::Bet),
        "stage must auto-advance to Bet, got {:?}",
        body.status,
    );
}

#[tokio::test]
async fn test_get_round_rolls_through_multiple_stages_at_once() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Backdate so far that NewsReveal + Bet + Rationale + Reveal all
    // elapsed in one go. Defaults sum to ~90s — backdate 5 minutes
    // to land squarely inside Debate (PR5's terminal stage).
    backdate_stage_deadline(&ctx, &round_id, 5 * 60 * 1000).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(
        matches!(body.status, RoundStatus::Debate),
        "PR5 stops auto-advance at Debate, got {:?}",
        body.status,
    );
}

#[tokio::test]
async fn test_bet_succeeds_after_lazy_advance_into_bet_stage() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Rewind so NewsReveal just expired — the bet handler should
    // ratchet into Bet *inside the handler* and accept the bet.
    backdate_stage_deadline(&ctx, &round_id, 1_000).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets", round_id),
        headers: headers,
        body: { "req": { "side": "REAL", "amount_rp": 100 } },
        response_type: BetResponse,
    };
    assert_eq!(status, 200, "bet placement after lazy advance must succeed: {:?}", body);
    assert!(matches!(body.side, BetSide::Real));
    assert_eq!(body.amount_rp, 100);
}

#[tokio::test]
async fn test_rationale_succeeds_after_lazy_advance_into_rationale_stage() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Backdate enough to roll NewsReveal + Bet into the past and
    // land in Rationale. Defaults: 30s + 10s = 40s → backdate 45s
    // ago (so NewsReveal expired 45s ago, Bet expired 35s ago,
    // landing the wall clock 35s into the 30s Rationale window).
    backdate_stage_deadline(&ctx, &round_id, 35_000).await;

    let rationale_text = "x".repeat(60); // > 50 chars → essence_eligible
    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/rationale", round_id),
        headers: headers,
        body: { "req": { "text": rationale_text } },
        response_type: RationaleResponse,
    };
    assert_eq!(status, 200, "rationale post after lazy advance must succeed: {:?}", body);
    assert!(body.essence_eligible, ">= 50-char rationale should be eligible");
}

// ── Chat (PR4f) ─────────────────────────────────────────────────────

#[tokio::test]
async fn test_post_chat_rejects_non_participant() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;

    let (_, outsider) = ctx.create_another_user().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/arcade/games/fact-or-fold/rounds/{}/chat", round_id),
        headers: outsider,
        body: { "req": { "text": "hello" } }
    };
    assert_ne!(status, 200, "outsider must not be able to post chat");
}

#[tokio::test]
async fn test_post_chat_rejects_outside_debate_stage() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Round is freshly started → NewsReveal, not Debate. Chat must
    // be rejected.
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/arcade/games/fact-or-fold/rounds/{}/chat", round_id),
        headers: headers,
        body: { "req": { "text": "hi" } }
    };
    assert_ne!(status, 200, "chat outside Debate stage must be rejected");
}

#[tokio::test]
async fn test_chat_polling_returns_empty_for_fresh_round() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/arcade/games/fact-or-fold/rounds/{}/chat", round_id),
        headers: headers,
        response_type: crate::features::arcade::games::fact_or_fold::types::ListChatResponse,
    };
    assert_eq!(status, 200);
    assert!(body.items.is_empty());
    assert!(body.last_id.is_none());
}

#[tokio::test]
async fn test_chat_polling_rejects_non_participant() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;

    let (_, outsider) = ctx.create_another_user().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/arcade/games/fact-or-fold/rounds/{}/chat", round_id),
        headers: outsider,
    };
    assert_ne!(status, 200, "outsider must not be able to read chat");
}

#[tokio::test]
async fn test_post_chat_rejects_empty_or_overlong() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/arcade/games/fact-or-fold/rounds/{}/chat", round_id),
        headers: headers.clone(),
        body: { "req": { "text": "" } }
    };
    assert_ne!(status, 200, "empty chat must be rejected");

    let too_long = "x".repeat(81);
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/arcade/games/fact-or-fold/rounds/{}/chat", round_id),
        headers: headers,
        body: { "req": { "text": too_long } }
    };
    assert_ne!(status, 200, "81-char chat must be rejected (>80 cap)");
}

// ── Flip slot (PR5) ─────────────────────────────────────────────────

/// Force the round into `Debate` stage with `remaining_ms` left on
/// the clock. Used to exercise both flip-slot-open and
/// flip-slot-closed branches without depending on real elapsed time.
async fn force_round_to_debate(ctx: &TestContext, round_id: &str, remaining_ms: i64) {
    let (pk, sk) = FactFoldRound::keys(round_id);
    let mut row = FactFoldRound::get(&ctx.ddb, &pk, Some(sk))
        .await
        .expect("ddb read")
        .expect("round must exist");
    let now = crate::common::utils::time::get_now_timestamp_millis();
    row.status = crate::features::arcade::games::fact_or_fold::types::RoundStatus::Debate;
    row.stage_started_at = Some(now - 1_000);
    row.stage_deadline_at = Some(now + remaining_ms);
    row.upsert(&ctx.ddb).await.expect("debate stage upsert");
}

/// Drop a `FactFoldBet` row directly into DDB so flip tests don't
/// have to walk the round through NewsReveal → Bet → place a bet.
async fn seed_bet(ctx: &TestContext, round_id: &str, user_pk: &Partition, side: &str) {
    use crate::features::arcade::games::fact_or_fold::models::FactFoldBet;
    use crate::features::arcade::games::fact_or_fold::types::BetSide;
    let bet_side = match side {
        "REAL" => BetSide::Real,
        "FAKE" => BetSide::Fake,
        _ => BetSide::Real,
    };
    let row = FactFoldBet::new(round_id, user_pk.clone(), bet_side, 100);
    row.upsert(&ctx.ddb).await.expect("seed bet");
}

/// Drop a rationale row so flip tests can cite it without going
/// through the rationale stage.
async fn seed_rationale(ctx: &TestContext, round_id: &str, user_pk: &Partition) {
    use crate::features::arcade::games::fact_or_fold::models::FactFoldRationale;
    let row = FactFoldRationale::new(round_id, user_pk.clone(), "valid rationale text".into(), true);
    row.upsert(&ctx.ddb).await.expect("seed rationale");
}

#[tokio::test]
async fn test_flip_rejected_outside_debate_stage() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    // Round is in NewsReveal.
    let cite = format!("USER#{}", uuid::Uuid::new_v4());
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets/flip", round_id),
        headers: headers,
        body: { "req": { "side": "FAKE", "cite_user_pk": cite } }
    };
    assert_ne!(status, 200, "flip outside Debate must be rejected");
}

#[tokio::test]
async fn test_flip_rejected_when_slot_closed_too_early() {
    // Debate stage but >10s remaining → slot still closed.
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bet(&ctx, &round_id, &ctx.test_user.0.pk, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 30_000).await; // 30s remaining

    let cite = format!("USER#{}", uuid::Uuid::new_v4());
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets/flip", round_id),
        headers: headers,
        body: { "req": { "side": "FAKE", "cite_user_pk": cite } }
    };
    assert_ne!(
        status, 200,
        "flip outside the last-10s window must be rejected"
    );
}

#[tokio::test]
async fn test_flip_rejected_self_cite() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bet(&ctx, &round_id, &ctx.test_user.0.pk, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;

    let self_pk = ctx.test_user.0.pk.to_string();
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets/flip", round_id),
        headers: headers,
        body: { "req": { "side": "FAKE", "cite_user_pk": self_pk } }
    };
    assert_ne!(status, 200, "self-citation must be rejected");
}

#[tokio::test]
async fn test_flip_rejected_when_cite_has_no_rationale() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bet(&ctx, &round_id, &ctx.test_user.0.pk, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;

    // Pick a real participant (the 2nd joiner) but don't seed their
    // rationale — flip must be rejected.
    let (pk, _) = FactFoldRound::keys(&round_id);
    let round = FactFoldRound::get(&ctx.ddb, &pk, Some(crate::common::types::EntityType::FactFoldRound(round_id.clone())))
        .await
        .expect("ddb read")
        .expect("round must exist");
    let other_pk = round.participant_pks
        .iter()
        .find(|p| p.to_string() != ctx.test_user.0.pk.to_string())
        .expect("must have another participant")
        .to_string();

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets/flip", round_id),
        headers: headers,
        body: { "req": { "side": "FAKE", "cite_user_pk": other_pk } }
    };
    assert_ne!(status, 200, "cite without rationale must be rejected");
}

#[tokio::test]
async fn test_flip_succeeds_in_last_10s_with_valid_cite() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bet(&ctx, &round_id, &ctx.test_user.0.pk, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;

    let (pk, _) = FactFoldRound::keys(&round_id);
    let round = FactFoldRound::get(&ctx.ddb, &pk, Some(crate::common::types::EntityType::FactFoldRound(round_id.clone())))
        .await
        .expect("ddb read")
        .expect("round must exist");
    let other_pk = round.participant_pks
        .iter()
        .find(|p| p.to_string() != ctx.test_user.0.pk.to_string())
        .expect("must have another participant")
        .clone();
    seed_rationale(&ctx, &round_id, &other_pk).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets/flip", round_id),
        headers: headers,
        body: { "req": { "side": "FAKE", "cite_user_pk": other_pk.to_string() } },
        response_type: crate::features::arcade::games::fact_or_fold::types::FlipBetResponse,
    };
    assert_eq!(status, 200, "valid flip must succeed: {:?}", body);
    assert!(matches!(body.original_side, BetSide::Real));
    assert!(matches!(body.flipped_to, BetSide::Fake));
}

#[tokio::test]
async fn test_flip_rejected_when_already_used() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bet(&ctx, &round_id, &ctx.test_user.0.pk, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;

    let (pk, _) = FactFoldRound::keys(&round_id);
    let round = FactFoldRound::get(&ctx.ddb, &pk, Some(crate::common::types::EntityType::FactFoldRound(round_id.clone())))
        .await
        .expect("ddb read")
        .expect("round must exist");
    let other_pk = round.participant_pks
        .iter()
        .find(|p| p.to_string() != ctx.test_user.0.pk.to_string())
        .expect("must have another participant")
        .clone();
    seed_rationale(&ctx, &round_id, &other_pk).await;

    // First flip — succeeds.
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/rounds/{}/bets/flip", round_id),
        headers: headers.clone(),
        body: { "req": { "side": "FAKE", "cite_user_pk": other_pk.to_string() } }
    };
    assert_eq!(status, 200);

    // Second flip — rejected.
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets/flip", round_id),
        headers: headers,
        body: { "req": { "side": "REAL", "cite_user_pk": other_pk.to_string() } }
    };
    assert_ne!(status, 200, "second flip in the same round must be rejected");
}

// ── Essence opt-in (PR6 step 4) ─────────────────────────────────────

#[tokio::test]
async fn test_essence_register_rejects_unregister_request() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!(
            "/api/arcade/games/fact-or-fold/rounds/{}/essence",
            round_id
        ),
        headers: headers,
        body: { "req": { "register": false } }
    };
    assert_ne!(status, 200, "register=false is not supported in v1");
}

#[tokio::test]
async fn test_essence_register_rejects_no_rationale() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    // Caller has no rationale row → reject.
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!(
            "/api/arcade/games/fact-or-fold/rounds/{}/essence",
            round_id
        ),
        headers: headers,
        body: { "req": { "register": true } }
    };
    assert_ne!(status, 200, "no rationale to register must be rejected");
}

#[tokio::test]
async fn test_essence_register_rejects_short_rationale() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Seed a too-short rationale (essence_eligible = false).
    use crate::features::arcade::games::fact_or_fold::models::FactFoldRationale;
    let row = FactFoldRationale::new(
        &round_id,
        ctx.test_user.0.pk.clone(),
        "too short".into(),
        false, // essence_eligible: false (< 50 chars)
    );
    row.upsert(&ctx.ddb).await.expect("seed short rationale");

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!(
            "/api/arcade/games/fact-or-fold/rounds/{}/essence",
            round_id
        ),
        headers: headers,
        body: { "req": { "register": true } }
    };
    assert_ne!(status, 200, "essence-ineligible rationale must be rejected");
}

// ── Stats + leaderboard (PR7) ───────────────────────────────────────

#[tokio::test]
async fn test_me_stats_returns_zero_for_new_user() {
    let ctx = TestContext::setup().await;
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/me/stats",
        headers: ctx.test_user.1.clone(),
        response_type: crate::features::arcade::games::fact_or_fold::types::UserStatsResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.total_rounds, 0);
    assert_eq!(body.correct_count, 0);
    assert_eq!(body.accuracy_bps, 0);
}

#[tokio::test]
async fn test_leaderboard_returns_empty_when_no_settlements() {
    let ctx = TestContext::setup().await;
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/leaderboard",
        headers: ctx.test_user.1.clone(),
        response_type: crate::common::types::ListResponse<
            crate::features::arcade::games::fact_or_fold::types::LeaderboardEntryResponse,
        >,
    };
    assert_eq!(status, 200);
    assert!(body.items.is_empty());
}

#[tokio::test]
async fn test_me_stats_after_settlement_reflects_round() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    // Everyone bets REAL — caller wins (headline default verdict
    // is REAL).
    seed_bets_for_all(&ctx, &round_id, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: admin,
    };
    assert_eq!(status, 200);

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/fact-or-fold/me/stats",
        headers: headers,
        response_type: crate::features::arcade::games::fact_or_fold::types::UserStatsResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.total_rounds, 1);
    assert_eq!(body.correct_count, 1);
    assert_eq!(body.accuracy_bps, 10_000, "100% after 1/1 correct");
}

// ── Settlement (PR6) ────────────────────────────────────────────────

/// Seed bets for every participant (4 in a full-capacity round)
/// using a single side. Useful for setting up a clean
/// winners/losers split before calling settle.
async fn seed_bets_for_all(ctx: &TestContext, round_id: &str, side: &str) {
    let (pk, sk) = FactFoldRound::keys(round_id);
    let round = FactFoldRound::get(&ctx.ddb, &pk, Some(sk))
        .await
        .expect("ddb read")
        .expect("round must exist");
    for p in round.participant_pks.iter() {
        seed_bet(ctx, round_id, p, side).await;
    }
}

#[tokio::test]
async fn test_settle_round_admin_only() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;

    // Non-admin must be rejected.
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: headers,
    };
    assert_ne!(status, 200, "non-admin must not be able to settle");

    // Admin succeeds.
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: admin,
    };
    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_settle_round_marks_round_settled() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    // Note: default headline verdict from `create_headline` is REAL.
    // All 4 bet REAL → everyone wins, no loser pool.
    seed_bets_for_all(&ctx, &round_id, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: admin,
    };
    assert_eq!(status, 200);

    // Round is now Settled.
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(
        matches!(body.status, RoundStatus::Settled),
        "round must be Settled after settle handler, got {:?}",
        body.status,
    );
    assert!(body.settled_at.is_some());
}

#[tokio::test]
async fn test_settle_round_idempotent() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bets_for_all(&ctx, &round_id, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;

    // First settle.
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: admin.clone(),
    };
    assert_eq!(status, 200);

    // Second settle — must also 200, must not double-credit.
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: admin,
    };
    assert_eq!(status, 200, "second settle on a Settled round must be a no-op");
}

#[tokio::test]
async fn test_chat_post_succeeds_during_debate() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    force_round_to_debate(&ctx, &round_id, 30_000).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/arcade/games/fact-or-fold/rounds/{}/chat", round_id),
        headers: headers,
        body: { "req": { "text": "hello debate" } }
    };
    assert_eq!(status, 200, "chat post during Debate must succeed");
}

// ── Client tick (PR4d) ──────────────────────────────────────────────

#[tokio::test]
async fn test_tick_before_deadline_is_noop() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Round freshly started → NewsReveal deadline ~30s in the future.
    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/tick", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(
        matches!(body.status, RoundStatus::NewsReveal),
        "tick before deadline must keep stage = NewsReveal, got {:?}",
        body.status,
    );
}

#[tokio::test]
async fn test_tick_after_deadline_advances_stage() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // NewsReveal expired 5s ago → tick should advance to Bet.
    backdate_stage_deadline(&ctx, &round_id, 5_000).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/tick", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(
        matches!(body.status, RoundStatus::Bet),
        "tick after NewsReveal deadline must advance to Bet, got {:?}",
        body.status,
    );
}

#[tokio::test]
async fn test_tick_rolls_through_multiple_stages() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Walk all the way to Debate in one tick (PR5 terminal stage).
    backdate_stage_deadline(&ctx, &round_id, 5 * 60 * 1000).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/tick", round_id),
        headers: headers,
        response_type: RoundResponse,
    };
    assert_eq!(status, 200);
    assert!(
        matches!(body.status, RoundStatus::Debate),
        "tick walks to PR5 terminal Debate, got {:?}",
        body.status,
    );
}

#[tokio::test]
async fn test_tick_rejects_non_participant() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;

    let (_, outsider) = ctx.create_another_user().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/tick", round_id),
        headers: outsider,
    };
    assert_ne!(status, 200, "non-participant must not be able to tick");
}

// ── Round-read endpoints (PR8 — game-room data feed) ────────────────

#[tokio::test]
async fn test_round_headline_redacts_verdict_until_settled() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Before settlement: verdict is None and reveal sources are empty.
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/rounds/{}/headline", round_id),
        headers: headers.clone(),
        response_type: RoundHeadlineResponse,
    };
    assert_eq!(status, 200);
    assert!(body.verdict.is_none(), "verdict must be hidden pre-settle");
    assert!(body.reveal_summary.is_empty(), "reveal_summary must be hidden pre-settle");
    assert!(!body.headline_text.is_empty(), "public headline text must be visible");

    // Settle the round.
    seed_bets_for_all(&ctx, &round_id, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: admin,
    };
    assert_eq!(status, 200);

    // After settlement: verdict + summary surface.
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/headline", round_id),
        headers: headers,
        response_type: RoundHeadlineResponse,
    };
    assert_eq!(status, 200);
    assert!(
        matches!(body.verdict, Some(Verdict::Real)),
        "verdict must surface after settle, got {:?}",
        body.verdict,
    );
    assert_eq!(body.reveal_summary, "Confirmed by source.");
}

#[tokio::test]
async fn test_round_headline_rejects_non_participant() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;

    let (_, outsider) = ctx.create_another_user().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/headline", round_id),
        headers: outsider,
    };
    assert_ne!(status, 200, "non-participant must not see the round headline");
}

#[tokio::test]
async fn test_round_bets_only_returns_own_during_bet_stage() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bets_for_all(&ctx, &round_id, "REAL").await;

    // Round is still NewsReveal — caller is a participant and their
    // bet row exists (seeded), but other players' rows are hidden.
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets", round_id),
        headers: headers,
        response_type: ListBetsResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1, "pre-reveal must only contain caller's bet");
    assert_eq!(body.items[0].user_pk, ctx.test_user.0.pk.to_string());
}

#[tokio::test]
async fn test_round_bets_returns_all_at_reveal_or_later() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bets_for_all(&ctx, &round_id, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 30_000).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets", round_id),
        headers: headers,
        response_type: ListBetsResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 4, "Debate stage must expose all 4 bets");
}

#[tokio::test]
async fn test_round_bets_rejects_non_participant() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;
    let (_, outsider) = ctx.create_another_user().await;

    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/bets", round_id),
        headers: outsider,
    };
    assert_ne!(status, 200, "non-participant must not see bets");
}

#[tokio::test]
async fn test_round_rationales_redacts_text_until_reveal() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Seed rationales for every player directly. Round is still in
    // NewsReveal — other players' text must be redacted, caller's
    // own text stays.
    let (round_pk, round_sk) = FactFoldRound::keys(&round_id);
    let round = FactFoldRound::get(&ctx.ddb, &round_pk, Some(round_sk))
        .await
        .expect("ddb read")
        .expect("round must exist");
    for p in round.participant_pks.iter() {
        seed_rationale(&ctx, &round_id, p).await;
    }

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/rounds/{}/rationale", round_id),
        headers: headers.clone(),
        response_type: ListRationalesResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 4, "all 4 rationale rows present");
    let own = body
        .items
        .iter()
        .find(|r| r.user_pk == ctx.test_user.0.pk.to_string())
        .expect("caller's row must be present");
    assert!(!own.text.is_empty(), "caller sees own rationale text");
    let others_with_text = body
        .items
        .iter()
        .filter(|r| r.user_pk != ctx.test_user.0.pk.to_string() && !r.text.is_empty())
        .count();
    assert_eq!(
        others_with_text, 0,
        "other players' rationale text must be redacted before Reveal",
    );

    // After advancing to Debate, all texts become visible.
    force_round_to_debate(&ctx, &round_id, 30_000).await;
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/rationale", round_id),
        headers: headers,
        response_type: ListRationalesResponse,
    };
    assert_eq!(status, 200);
    assert!(
        body.items.iter().all(|r| !r.text.is_empty()),
        "all 4 rationales must have text at Reveal+",
    );
}

#[tokio::test]
async fn test_round_participants_lists_with_display_metadata() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/participants", round_id),
        headers: headers,
        response_type: ListParticipantsResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 4);
    // Exactly one row may carry is_insider=true (the caller's own row,
    // and only if the matcher rolled them as insider). Other rows must
    // always report false even if that player is the insider.
    let visible_insiders = body.items.iter().filter(|p| p.is_insider).count();
    assert!(
        visible_insiders <= 1,
        "is_insider must be redacted to at-most-one row (caller's own)",
    );
}

#[tokio::test]
async fn test_round_participants_rejects_non_participant() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, _) = fill_round_to_capacity(&ctx, &admin).await;
    let (_, outsider) = ctx.create_another_user().await;

    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/participants", round_id),
        headers: outsider,
    };
    assert_ne!(status, 200, "non-participant must not see roster");
}

#[tokio::test]
async fn test_round_settlement_rejects_when_not_settled() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;

    // Round is fresh — not yet settled.
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/settlement", round_id),
        headers: headers,
    };
    assert_ne!(status, 200, "settlement read must fail before round is Settled");
}

#[tokio::test]
async fn test_round_settlement_returns_breakdown_after_settle() {
    let ctx = TestContext::setup().await;
    let (_, admin) = ctx.create_admin_user().await;
    let (round_id, headers) = fill_round_to_capacity(&ctx, &admin).await;
    seed_bets_for_all(&ctx, &round_id, "REAL").await;
    force_round_to_debate(&ctx, &round_id, 5_000).await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/fact-or-fold/admin/rounds/{}/settle", round_id),
        headers: admin,
    };
    assert_eq!(status, 200);

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/fact-or-fold/rounds/{}/settlement", round_id),
        headers: headers,
        response_type: SettleRoundResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.round_id, round_id);
    assert_eq!(body.outcomes.len(), 4, "one breakdown per participant");
}

#[allow(dead_code)]
fn _force_dto_imports_used() {
    // Pulls in BetResponse / RationaleResponse / BetSide so they
    // don't fire unused-import warnings in this tests file even
    // though they are exercised primarily via JSON shape (no direct
    // typed parse needed for the negative-path tests above).
    let _ = std::any::type_name::<BetResponse>();
    let _ = std::any::type_name::<RationaleResponse>();
    let _ = std::any::type_name::<BetSide>();
    let _ = std::any::type_name::<FactOrFoldSettingsResponse>();
}
