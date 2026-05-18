//! Chat endpoints (PR4f).
//!
//! Surface:
//!   POST  /api/arcade/games/fact-or-fold/rounds/{round_id}/chat
//!   GET   /api/arcade/games/fact-or-fold/rounds/{round_id}/chat?since=...
//!
//! v1 only allows *posting* during the live debate stage (`Debate`).
//! Reads (the polling endpoint) are open to participants in any
//! stage so a freshly-joined client can render any backlog.
//!
//! ### v1 realtime — polling
//!
//! Per design doc § A2' (re-decision): chat in v1 uses HTTP short
//! polling, not SSE. The PR4e SSE infra + PR4f stream-listener
//! fan-out are kept in place but only fully come alive once the
//! Lambda binary supports streaming responses (v2 alongside the
//! WebSocket switch). Until then the client polls
//! `GET .../chat?since=<last_msg_id>` every 2~3s alongside the
//! round-state poll.

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{FactFoldChatMessage, FactFoldRound};
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::realtime::chat_payload_from;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::services::stage_machine;

#[post("/api/arcade/games/fact-or-fold/rounds/{round_id}/chat", user: User)]
pub async fn post_chat_handler(
    round_id: FactFoldRoundEntityType,
    req: PostChatRequest,
) -> Result<PostChatResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let (pk, sk) = FactFoldRound::keys(&inner_round_id);
    let round = FactFoldRound::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("post_chat_handler round read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    // Lazy advance so a stale client doesn't sneak chat past a
    // deadline that already passed.
    let settings = crate::features::arcade::games::fact_or_fold::models::FactFoldSettings::get_or_default(cli)
        .await
        .unwrap_or_default();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let round = stage_machine::advance_round_if_due(cli, round, &settings, now).await?;

    if !matches!(round.status, RoundStatus::Debate) {
        // Chat is only open during the live-debate stage. For PR4
        // (Debate not yet wired) this gate effectively blocks chat
        // until PR5 lands. The endpoint still exists so the SSE
        // subscribe flow can be exercised end-to-end against a
        // round that's been manually backdated into Debate.
        return Err(FactOrFoldError::RationaleStageMismatch.into());
    }

    let user_pk_str = user.pk.to_string();
    let in_round = round
        .participant_pks
        .iter()
        .any(|p| p.to_string() == user_pk_str);
    if !in_round {
        return Err(FactOrFoldError::NotRoundParticipant.into());
    }

    // Validate text shape.
    let len = req.text.chars().count();
    if len == 0 || len > CHAT_TEXT_MAX_CHARS {
        return Err(FactOrFoldError::RationaleInvalid.into());
    }

    // DB write. The DDB Stream / EventBridge fan-out (PR4f stream
    // listener wiring below) takes care of the SSE push.
    let row = FactFoldChatMessage::new(&inner_round_id, user.pk.clone(), req.text);
    row.create(cli).await.map_err(|e| {
        crate::error!("post_chat_handler create failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(PostChatResponse {
        msg_id: row.id().unwrap_or_default(),
        author_pk: row.author_pk.to_string(),
        text: row.text,
        sent_at: row.sent_at,
    })
}

// ── GET /api/arcade/games/fact-or-fold/rounds/{round_id}/chat ────
//
// v1 short-polling endpoint. Returns the chat transcript for a
// round, optionally filtered to messages newer than `since`
// (an opaque message id from the last response — same shape as
// `PostChatResponse.msg_id`).
//
// Only round participants can read. No stage gate on reads — a
// participant should be able to render backlog regardless of which
// stage the round is in.

#[get(
    "/api/arcade/games/fact-or-fold/rounds/{round_id}/chat?since",
    user: User
)]
pub async fn list_chat_handler(
    round_id: FactFoldRoundEntityType,
    since: Option<String>,
) -> Result<ListChatResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let (pk, sk) = FactFoldRound::keys(&inner_round_id);
    let round = FactFoldRound::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("list_chat_handler round read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    let user_pk_str = user.pk.to_string();
    let in_round = round
        .participant_pks
        .iter()
        .any(|p| p.to_string() == user_pk_str);
    if !in_round {
        return Err(FactOrFoldError::NotRoundParticipant.into());
    }

    // sk-prefix query orders chronologically (uuid_v7 inner). v1
    // uses a single-page fetch — round transcripts are bounded
    // (~4 players × 70s × 80 chars).
    let opts = FactFoldChatMessage::opt()
        .sk("FACT_FOLD_CHAT".to_string())
        .limit(CHAT_PAGE_LIMIT as i32);
    let (rows, _) = FactFoldChatMessage::query(cli, pk, opts)
        .await
        .map_err(|e| {
            crate::error!("list_chat_handler query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let items: Vec<ChatMessagePayload> = match since {
        Some(after) if !after.is_empty() => rows
            .into_iter()
            .skip_while(|r| r.id().map(|id| id <= after).unwrap_or(true))
            .map(chat_payload_from)
            .collect(),
        _ => rows.into_iter().map(chat_payload_from).collect(),
    };

    let last_id = items.last().map(|m| m.msg_id.clone());
    Ok(ListChatResponse { items, last_id })
}

// ── DELETE /api/arcade/games/fact-or-fold/rounds/{round_id}/chat ──
//
// Bulk-delete the full chat transcript for a round. Today this is
// called by the settlement-screen "정산완료 → 홈으로" button after
// the player has reviewed the breakdown. Guarded by:
//   - caller is a participant of the round
//   - round is Settled (no live messages get nuked)
//
// Permanent transcript retention is documented as a roadmap §FR-11
// goal; we ship this temporary cleanup until the long-term policy
// (TTL? summarize-then-delete?) lands. Idempotent: a second call by
// another participant after the rows are gone returns OK with 0.

#[delete("/api/arcade/games/fact-or-fold/rounds/{round_id}/chat", user: User)]
pub async fn delete_round_chat_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<DeleteRoundChatResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let (pk, sk) = FactFoldRound::keys(&inner_round_id);
    let round = FactFoldRound::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("delete_round_chat_handler round read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    let user_pk_str = user.pk.to_string();
    let in_round = round
        .participant_pks
        .iter()
        .any(|p| p.to_string() == user_pk_str);
    if !in_round {
        return Err(FactOrFoldError::NotRoundParticipant.into());
    }

    if !matches!(round.status, RoundStatus::Settled) {
        return Err(FactOrFoldError::RoundNotSettled.into());
    }

    // Single pk scan to enumerate the transcript, then delete each
    // row individually. DDB's BatchWriteItem (25 per call) would be
    // faster but v1 transcripts are bounded so the simple per-row
    // delete keeps the code obvious — revisit when transcript bound
    // is lifted.
    let opts = FactFoldChatMessage::opt()
        .sk("FACT_FOLD_CHAT".to_string())
        .limit(CHAT_PAGE_LIMIT as i32);
    let (rows, _) = FactFoldChatMessage::query(cli, pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("delete_round_chat_handler query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let mut deleted = 0i64;
    for row in rows.iter() {
        if let Err(e) = FactFoldChatMessage::delete(cli, &row.pk, Some(row.sk.clone())).await {
            crate::error!("delete_round_chat_handler row delete failed: {e}");
            continue;
        }
        deleted += 1;
    }

    Ok(DeleteRoundChatResponse { deleted })
}
