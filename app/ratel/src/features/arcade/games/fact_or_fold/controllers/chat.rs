//! Chat endpoint (PR4f).
//!
//! Surface:
//!   POST  /api/arcade/games/fact-or-fold/rounds/{round_id}/chat
//!
//! v1 only allows posting during the live debate stage (`Debate`).
//! v1 fan-out: the write-only controller persists a
//! `FactFoldChatMessage`. The DDB Stream listener detects the
//! INSERT and triggers `arcade::realtime::global_hub().publish(...)`
//! so every SSE invocation broadcasts the message to its own
//! subscribers (design doc § A2'/A12).

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{FactFoldChatMessage, FactFoldRound};
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
