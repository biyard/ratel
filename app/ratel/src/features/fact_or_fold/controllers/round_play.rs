//! Round-play endpoints (PR4 step 2).
//!
//! Surface:
//!   POST  /api/fact-or-fold/rounds/{round_id}/bets
//!   POST  /api/fact-or-fold/rounds/{round_id}/rationale
//!   GET   /api/fact-or-fold/rounds/{round_id}/insider-statement
//!   POST  /api/fact-or-fold/rounds/{round_id}/heartbeat
//!
//! All endpoints validate (a) the caller is a round participant and
//! (b) the round is in the right stage. Insider statement is gated
//! by `FactFoldParticipant.is_insider` per design doc § Insider
//! protection — the response always wraps `Option<String>` so the
//! "not insider" branch is a normal Some/None, not a 403.

use crate::common::*;
use crate::features::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::fact_or_fold::models::{
    FactFoldBet, FactFoldHeadline, FactFoldParticipant, FactFoldRationale, FactFoldRound,
    FactFoldSettings,
};

// ── Helpers ───────────────────────────────────────────────────────

#[cfg(feature = "server")]
async fn load_round_or_404(
    cli: &aws_sdk_dynamodb::Client,
    round_id: &str,
) -> Result<FactFoldRound> {
    let (pk, sk) = FactFoldRound::keys(round_id);
    FactFoldRound::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("round_play load_round failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or_else(|| FactOrFoldError::RoundNotFound.into())
}

#[cfg(feature = "server")]
fn ensure_participant(round: &FactFoldRound, user_pk_str: &str) -> Result<()> {
    let in_round = round
        .participant_pks
        .iter()
        .any(|p| p.to_string() == user_pk_str);
    if !in_round {
        return Err(FactOrFoldError::NotRoundParticipant.into());
    }
    Ok(())
}

#[cfg(feature = "server")]
async fn load_participant(
    cli: &aws_sdk_dynamodb::Client,
    round_id: &str,
    user_pk: &Partition,
) -> Result<FactFoldParticipant> {
    let user_id = user_pk
        .to_string()
        .strip_prefix("USER#")
        .unwrap_or(&user_pk.to_string())
        .to_string();
    let (pk, sk) = FactFoldParticipant::keys(round_id, &user_id);
    FactFoldParticipant::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("load_participant failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or_else(|| FactOrFoldError::NotRoundParticipant.into())
}

// ── POST /api/fact-or-fold/rounds/{round_id}/bets ────────────────

#[post("/api/fact-or-fold/rounds/{round_id}/bets", user: User)]
pub async fn place_bet_handler(
    round_id: FactFoldRoundEntityType,
    req: PlaceBetRequest,
) -> Result<BetResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_or_404(cli, &inner_round_id).await?;
    if !matches!(round.status, RoundStatus::Bet) {
        return Err(FactOrFoldError::BetStageMismatch.into());
    }
    ensure_participant(&round, &user.pk.to_string())?;

    let settings = FactFoldSettings::get_or_default(cli).await.unwrap_or_default();
    if req.amount_rp < settings.min_bet_rp || req.amount_rp > settings.max_bet_rp {
        return Err(FactOrFoldError::BetAmountOutOfRange.into());
    }

    let row = FactFoldBet::new(&inner_round_id, user.pk.clone(), req.side, req.amount_rp);
    row.upsert(cli).await.map_err(|e| {
        crate::error!("place_bet_handler upsert failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(BetResponse {
        user_pk: user.pk.to_string(),
        side: row.side,
        amount_rp: row.amount_rp,
        locked_at: row.locked_at,
        flipped_to: row.flipped_to,
        flip_cite_user_pk: row.flip_cite_user_pk.map(|p| p.to_string()),
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

// ── POST /api/fact-or-fold/rounds/{round_id}/rationale ───────────

#[post("/api/fact-or-fold/rounds/{round_id}/rationale", user: User)]
pub async fn submit_rationale_handler(
    round_id: FactFoldRoundEntityType,
    req: SubmitRationaleRequest,
) -> Result<RationaleResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_or_404(cli, &inner_round_id).await?;
    if !matches!(round.status, RoundStatus::Rationale) {
        return Err(FactOrFoldError::RationaleStageMismatch.into());
    }
    ensure_participant(&round, &user.pk.to_string())?;

    let len = req.text.chars().count();
    if len == 0 || len > RATIONALE_TEXT_MAX_CHARS {
        return Err(FactOrFoldError::RationaleInvalid.into());
    }
    let essence_eligible = len >= RATIONALE_ESSENCE_MIN_CHARS;

    let row = FactFoldRationale::new(&inner_round_id, user.pk.clone(), req.text, essence_eligible);
    row.upsert(cli).await.map_err(|e| {
        crate::error!("submit_rationale_handler upsert failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(RationaleResponse {
        user_pk: user.pk.to_string(),
        text: row.text,
        submitted_at: row.submitted_at,
        essence_eligible: row.essence_eligible,
        essence_registered: row.essence_registered,
    })
}

// ── GET /api/fact-or-fold/rounds/{round_id}/insider-statement ────

#[get("/api/fact-or-fold/rounds/{round_id}/insider-statement", user: User)]
pub async fn get_insider_statement_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<InsiderStatementResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk.to_string())?;

    let participant = load_participant(cli, &inner_round_id, &user.pk).await?;
    if !participant.is_insider {
        // Not the insider — return empty Option rather than 403 so
        // the UI can call this for everyone and just hide the panel
        // when statement is None.
        return Ok(InsiderStatementResponse { statement: None });
    }

    let pk = FactFoldHeadline::anchor_pk();
    let sk: EntityType = FactFoldHeadlineEntityType(round.headline_id.clone()).into();
    let headline = FactFoldHeadline::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_insider_statement_handler headline read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    Ok(InsiderStatementResponse {
        statement: Some(headline.insider_statement),
    })
}

// ── POST /api/fact-or-fold/rounds/{round_id}/heartbeat ───────────

#[post("/api/fact-or-fold/rounds/{round_id}/heartbeat", user: User)]
pub async fn heartbeat_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<ParticipantResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk.to_string())?;

    let user_id = user
        .pk
        .to_string()
        .strip_prefix("USER#")
        .unwrap_or(&user.pk.to_string())
        .to_string();
    let (pk, sk) = FactFoldParticipant::keys(&inner_round_id, &user_id);
    let mut participant = FactFoldParticipant::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("heartbeat_handler load failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::NotRoundParticipant)?;

    let now = crate::common::utils::time::get_now_timestamp_millis();
    participant.last_seen_at = now;
    participant.updated_at = now;
    participant.upsert(cli).await.map_err(|e| {
        crate::error!("heartbeat_handler upsert failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(ParticipantResponse {
        user_pk: user.pk.to_string(),
        joined_at: participant.joined_at,
        // Heartbeat returns the *caller's own* row, so is_insider is
        // safe to surface unredacted.
        is_insider: participant.is_insider,
        last_seen_at: participant.last_seen_at,
        forfeited: participant.forfeited,
    })
}
