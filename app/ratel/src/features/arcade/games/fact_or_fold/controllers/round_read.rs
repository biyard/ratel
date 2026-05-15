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
//!   GET /api/fact-or-fold/rounds/{round_id}/headline       — public headline
//!   GET /api/fact-or-fold/rounds/{round_id}/bets           — bet roster (gated)
//!   GET /api/fact-or-fold/rounds/{round_id}/rationale      — rationales (gated)
//!   GET /api/fact-or-fold/rounds/{round_id}/participants   — participants + display meta
//!   GET /api/fact-or-fold/rounds/{round_id}/settlement     — final breakdown (settled-only)

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::controllers::settlement::{
    settle_round_internal, SettleRoundResponse,
};
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{
    FactFoldBet, FactFoldHeadline, FactFoldParticipant, FactFoldRationale, FactFoldRound,
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

// ── GET /api/fact-or-fold/rounds/{round_id}/headline ──────────────

#[get("/api/fact-or-fold/rounds/{round_id}/headline", user: User)]
pub async fn get_round_headline_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<RoundHeadlineResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    let round = load_round_advanced_or_404(cli, &inner_round_id).await?;
    ensure_participant(&round, &user.pk.to_string())?;

    let pk = FactFoldHeadline::anchor_pk();
    let sk: EntityType = FactFoldHeadlineEntityType(round.headline_id.clone()).into();
    let headline = FactFoldHeadline::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_round_headline_handler read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    let settled = matches!(round.status, RoundStatus::Settled);
    Ok(RoundHeadlineResponse {
        id: FactFoldHeadlineEntityType(round.headline_id.clone()),
        headline_text: headline.headline_text,
        body_excerpt: headline.body_excerpt,
        source_label: headline.source_label,
        category_tags: headline.category_tags,
        difficulty: headline.difficulty,
        verdict: if settled { Some(headline.verdict) } else { None },
        reveal_summary: if settled { headline.reveal_summary } else { String::new() },
        reveal_sources: if settled { headline.reveal_sources } else { Vec::new() },
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
    let user_pk_str = user.pk.to_string();
    ensure_participant(&round, &user_pk_str)?;

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
        .filter(|r| unlocked || r.user_pk.to_string() == user_pk_str)
        .map(|r| BetResponse {
            user_pk: r.user_pk.to_string(),
            side: r.side,
            amount_rp: r.amount_rp,
            locked_at: r.locked_at,
            flipped_to: r.flipped_to,
            flip_cite_user_pk: r.flip_cite_user_pk.map(|p| p.to_string()),
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
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
    let user_pk_str = user.pk.to_string();
    ensure_participant(&round, &user_pk_str)?;

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
            let is_self = r.user_pk.to_string() == user_pk_str;
            let text = if unlocked || is_self {
                r.text
            } else {
                // Row stays so the UI can render the "submitted" pulse,
                // but the actual rationale text is redacted.
                String::new()
            };
            RationaleResponse {
                user_pk: r.user_pk.to_string(),
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
    let user_pk_str = user.pk.to_string();
    ensure_participant(&round, &user_pk_str)?;

    let (pk, _) = FactFoldRound::keys(&inner_round_id);
    let opts = FactFoldParticipant::opt()
        .sk("FACT_FOLD_PARTICIPANT".to_string())
        .limit(50);
    let (rows, _) = FactFoldParticipant::query(cli, pk, opts).await.map_err(|e| {
        crate::error!("list_round_participants_handler query failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    // Resolve display metadata per participant. v1 = 4 players max so
    // a per-row User::get is cheap; batch lookup is a follow-up if
    // the round size grows.
    let mut items: Vec<RoundParticipantSummary> = Vec::with_capacity(rows.len());
    for p in rows.into_iter() {
        let user_pk_owned = p.user_pk.clone();
        let user_row = User::get(cli, &user_pk_owned, Some(EntityType::User))
            .await
            .map_err(|e| {
                crate::error!("list_round_participants_handler user load failed: {e}");
                FactOrFoldError::StorageFailure
            })?;
        let (username, display_name, profile_url) = user_row
            .map(|u| (u.username, u.display_name, u.profile_url))
            .unwrap_or_default();
        let is_self = p.user_pk.to_string() == user_pk_str;
        items.push(RoundParticipantSummary {
            user_pk: p.user_pk.to_string(),
            username,
            display_name,
            profile_url,
            joined_at: p.joined_at,
            last_seen_at: p.last_seen_at,
            forfeited: p.forfeited,
            // Only echo the insider flag on the caller's own row —
            // protects the insider identity per design doc §Insider.
            is_insider: is_self && p.is_insider,
        });
    }

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
    ensure_participant(&round, &user.pk.to_string())?;

    if !matches!(round.status, RoundStatus::Settled) {
        return Err(FactOrFoldError::RoundNotSettled.into());
    }

    // settle_round_internal is idempotent: on an already-Settled round
    // it short-circuits and re-loads the persisted breakdowns. That's
    // exactly the read path we want — single source of truth for the
    // SettleRoundResponse shape.
    settle_round_internal(cli, &inner_round_id).await
}
