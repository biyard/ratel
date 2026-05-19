//! Player-side round read endpoints — the data feed that powers the
//! game-room views. Five GETs share one rule:
//!
//!   1. Caller must be a participant in the round.
//!   2. The round is `load_round_advanced_or_404`-ratchetted on every
//!      call (same lazy-advance gate as `round_play.rs`) so a stale
//!      client always observes the correct stage.
//!   3. Stage-sensitive fields are redacted until the round reaches
//!      the stage where the operator chose to surface them.
//!
//! Surface:
//!   GET /api/fact-or-fold/rounds/{round_id}/subject       — public subject
//!   GET /api/fact-or-fold/rounds/{round_id}/bets           — bet roster (gated)
//!   GET /api/fact-or-fold/rounds/{round_id}/rationale      — rationales (gated)
//!   GET /api/fact-or-fold/rounds/{round_id}/participants   — participants + display meta
//!   GET /api/fact-or-fold/rounds/{round_id}/settlement     — final breakdown (settled-only)

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::controllers::settlement::SettleRoundResponse;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::controllers::settlement::settle_round_internal;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{
    FactFoldBet, FactFoldSubject, FactFoldParticipant, FactFoldRationale, FactFoldRound,
    FactFoldSettings,
};
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::services::stage_machine;

// ── Shared helpers ────────────────────────────────────────────────

#[cfg(feature = "server")]
async fn load_round_advanced_or_404(
    cli: &aws_sdk_dynamodb::Client,
    round_id: &str,
) -> Result<FactFoldRound> {
    let (pk, sk) = FactFoldRound::keys(round_id);
    let round = FactFoldRound::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("round_read load_round failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;
    let settings = FactFoldSettings::get_or_default(cli)
        .await
        .unwrap_or_default();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    stage_machine::advance_round_if_due(cli, round, &settings, now).await
}

#[cfg(feature = "server")]
fn ensure_participant(round: &FactFoldRound, user_pk: &Partition) -> Result<()> {
    if !round.participant_pks.iter().any(|p| p == user_pk) {
        return Err(FactOrFoldError::NotRoundParticipant.into());
    }
    Ok(())
}

/// Bet/Rationale lists are public to the table only once the
/// `Reveal` stage opens — before then players see their own row
/// only.
#[cfg(feature = "server")]
fn full_reveal_unlocked(status: RoundStatus) -> bool {
    matches!(
        status,
        RoundStatus::Reveal
            | RoundStatus::Debate
            | RoundStatus::Settlement
            | RoundStatus::Settled
    )
}

// ── GET /api/fact-or-fold/rounds/{round_id}/subject ──────────────

#[get("/api/fact-or-fold/rounds/{round_id}/subject", user: User)]
pub async fn get_round_subject_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<RoundSubjectResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_advanced_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk)?;

    let pk = FactFoldSubject::anchor_pk();
    let sk: EntityType = FactFoldSubjectEntityType(round.subject_id.clone()).into();
    let subject = FactFoldSubject::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_round_subject_handler read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    let settled = matches!(round.status, RoundStatus::Settled);
    Ok(RoundSubjectResponse {
        id: FactFoldSubjectEntityType(round.subject_id.clone()),
        headline_text: subject.headline_text,
        body_excerpt: subject.body_excerpt,
        source_label: subject.source_label,
        category_tags: subject.category_tags,
        difficulty: subject.difficulty,
        verdict: if settled { Some(subject.verdict) } else { None },
        reveal_summary: if settled { subject.reveal_summary } else { String::new() },
        reveal_sources: if settled { subject.reveal_sources } else { Vec::new() },
    })
}

// ── GET /api/fact-or-fold/rounds/{round_id}/bets ──────────────────

#[get("/api/fact-or-fold/rounds/{round_id}/bets", user: User)]
pub async fn list_round_bets_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<ListBetsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_advanced_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk)?;

    let (pk, _) = FactFoldRound::keys(&inner_round_id);
    let opts = FactFoldBet::opt()
        .sk("FACT_FOLD_BET".to_string())
        .limit(50);
    let (rows, _) = FactFoldBet::query(cli, pk, opts).await.map_err(|e| {
        crate::error!("list_round_bets_handler query failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    let unlocked = full_reveal_unlocked(round.status);
    let items: Vec<BetResponse> = rows
        .into_iter()
        .filter(|r| unlocked || r.user_pk == user.pk)
        .map(|r| BetResponse::from(&r))
        .collect();

    Ok(ListBetsResponse { items })
}

// ── GET /api/fact-or-fold/rounds/{round_id}/rationale ─────────────

#[get("/api/fact-or-fold/rounds/{round_id}/rationale", user: User)]
pub async fn list_round_rationales_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<ListRationalesResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_advanced_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk)?;

    let (pk, _) = FactFoldRound::keys(&inner_round_id);
    let opts = FactFoldRationale::opt()
        .sk("FACT_FOLD_RATIONALE".to_string())
        .limit(50);
    let (rows, _) = FactFoldRationale::query(cli, pk, opts).await.map_err(|e| {
        crate::error!("list_round_rationales_handler query failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    let unlocked = full_reveal_unlocked(round.status);
    let items: Vec<RationaleResponse> = rows
        .into_iter()
        .map(|r| {
            let is_self = r.user_pk == user.pk;
            let text = if unlocked || is_self {
                r.text
            } else {
                // Row stays so the UI can render the "submitted" pulse,
                // but the actual rationale text is redacted.
                String::new()
            };
            RationaleResponse {
                user_pk: UserPartition::from(r.user_pk),
                text,
                submitted_at: r.submitted_at,
                essence_eligible: r.essence_eligible,
                essence_registered: r.essence_registered,
            }
        })
        .collect();

    Ok(ListRationalesResponse { items })
}

// ── GET /api/fact-or-fold/rounds/{round_id}/participants ──────────

#[get("/api/fact-or-fold/rounds/{round_id}/participants", user: User)]
pub async fn list_round_participants_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<ListParticipantsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_advanced_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk)?;

    let (pk, _) = FactFoldRound::keys(&inner_round_id);
    let opts = FactFoldParticipant::opt()
        .sk("FACT_FOLD_PARTICIPANT".to_string())
        .limit(50);
    let (rows, _) = FactFoldParticipant::query(cli, pk, opts).await.map_err(|e| {
        crate::error!("list_round_participants_handler query failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    // Resolve display metadata per participant via a single
    // BatchGetItem so the request is one RTT regardless of round
    // size. Missing rows fall back to empty defaults.
    let user_keys: Vec<(Partition, EntityType)> = rows
        .iter()
        .map(|p| (p.user_pk.clone(), EntityType::User))
        .collect();
    let user_rows = User::batch_get(cli, user_keys).await.map_err(|e| {
        crate::error!("list_round_participants_handler user batch load failed: {e}");
        FactOrFoldError::StorageFailure
    })?;
    // HashMap keyed on the rendered USER#{id} so we can index back by
    // `participant.user_pk.to_string()` — Partition itself doesn't
    // derive Hash, and adding it lives in a shared types crate.
    let user_by_pk: std::collections::HashMap<String, User> = user_rows
        .into_iter()
        .map(|u| (u.pk.to_string(), u))
        .collect();

    let items: Vec<RoundParticipantSummary> = rows
        .into_iter()
        .map(|p| {
            let (username, display_name, profile_url) = user_by_pk
                .get(&p.user_pk.to_string())
                .map(|u| (u.username.clone(), u.display_name.clone(), u.profile_url.clone()))
                .unwrap_or_default();
            let is_self = p.user_pk == user.pk;
            RoundParticipantSummary {
                user_pk: UserPartition::from(p.user_pk),
                username,
                display_name,
                profile_url,
                joined_at: p.joined_at,
                last_seen_at: p.last_seen_at,
                forfeited: p.forfeited,
                // Only echo the insider flag on the caller's own row —
                // protects the insider identity per design doc §Insider.
                is_insider: is_self && p.is_insider,
            }
        })
        .collect();

    Ok(ListParticipantsResponse { items })
}

// ── GET /api/fact-or-fold/rounds/{round_id}/settlement ────────────

#[get("/api/fact-or-fold/rounds/{round_id}/settlement", user: User)]
pub async fn get_round_settlement_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<SettleRoundResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_advanced_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk)?;

    if !matches!(round.status, RoundStatus::Settled) {
        return Err(FactOrFoldError::RoundNotSettled.into());
    }

    // settle_round_internal is idempotent: on an already-Settled round
    // it short-circuits and re-loads the persisted breakdowns. That's
    // exactly the read path we want — single source of truth for the
    // SettleRoundResponse shape.
    settle_round_internal(cli, &inner_round_id).await
}
