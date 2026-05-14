//! Stage state machine for *Fact or Fold* rounds.
//!
//! Server-verified by time (§FR-9): clients cannot forge stage state
//! to skip ahead. The chain is one-way and monotonic:
//!
//!   `NewsReveal` → `Bet` → `Rationale` → `Reveal` → (Debate, PR5)
//!
//! PR4 wires NewsReveal → Bet → Rationale → Reveal only. The
//! `Reveal → Debate` hand-off lands in PR5; the round sits in
//! `Reveal` once its deadline passes here.
//!
//! ### Advancement strategy
//!
//! v1 is "lazy": every read/write on a round calls
//! [`advance_round_if_due`] first, so any participant interaction
//! ratchets the stage forward up to the current wall-clock. PR4's
//! follow-up commits add an EventBridge scheduled trigger as the
//! *primary* advance signal; the lazy path stays as a safety net so
//! a request that races the scheduled event still sees the correct
//! stage.

use crate::features::fact_or_fold::models::FactFoldRound;
use crate::features::fact_or_fold::types::{
    FactOrFoldError, FactOrFoldSettingsResponse, RoundStatus,
};

/// Successor stage in the auto-progression chain (§FR-7). Returns
/// `None` for stages PR4 doesn't drive (Waiting, Reveal→?, Debate→?,
/// Settlement→?, Settled).
pub fn next_stage(current: RoundStatus) -> Option<RoundStatus> {
    match current {
        RoundStatus::NewsReveal => Some(RoundStatus::Bet),
        RoundStatus::Bet => Some(RoundStatus::Rationale),
        RoundStatus::Rationale => Some(RoundStatus::Reveal),
        // Reveal → Debate is PR5; Debate → Settlement is PR5/PR6;
        // Settlement → Settled is PR6. The round sits in `Reveal`
        // until those PRs land.
        _ => None,
    }
}

/// Stage duration in milliseconds, sourced from admin-tunable
/// [`crate::features::fact_or_fold::types::FactOrFoldSettingsResponse`].
/// Returns `None` for stages without a fixed countdown (Waiting,
/// Settlement, Settled).
pub fn stage_duration_ms(
    stage: RoundStatus,
    s: &FactOrFoldSettingsResponse,
) -> Option<i64> {
    let secs = match stage {
        RoundStatus::NewsReveal => s.stage_news_reveal_sec,
        RoundStatus::Bet => s.stage_bet_sec,
        RoundStatus::Rationale => s.stage_rationale_sec,
        RoundStatus::Reveal => s.stage_reveal_sec,
        RoundStatus::Debate => s.stage_debate_sec,
        RoundStatus::Waiting | RoundStatus::Settlement | RoundStatus::Settled => {
            return None;
        }
    };
    Some((secs as i64) * 1000)
}

/// Stamp the initial stage clock on the round. Called by the lobby
/// matching service exactly once, on the Waiting → NewsReveal
/// transition.
pub fn stamp_initial_stage(
    round: &mut FactFoldRound,
    settings: &FactOrFoldSettingsResponse,
    now_ms: i64,
) {
    round.stage_started_at = Some(now_ms);
    round.stage_deadline_at = stage_duration_ms(round.status, settings).map(|d| now_ms + d);
}

/// Walk the round forward through every stage whose deadline has
/// already passed at `now_ms`. Stops at the first stage that is
/// either still in its window or that PR4 doesn't auto-advance past
/// (currently `Reveal`).
///
/// Persists at most once at the end of the loop (single upsert per
/// call regardless of how many stages were crossed). The deadline
/// of the new current stage uses the *previous* deadline as its
/// anchor — not `now_ms` — so total round length stays bounded by
/// the sum of stage durations even if the caller is late.
pub async fn advance_round_if_due(
    cli: &aws_sdk_dynamodb::Client,
    mut round: FactFoldRound,
    settings: &FactOrFoldSettingsResponse,
    now_ms: i64,
) -> crate::common::Result<FactFoldRound> {
    let mut advanced = false;
    loop {
        let Some(deadline) = round.stage_deadline_at else {
            break;
        };
        if now_ms < deadline {
            break;
        }
        let Some(next) = next_stage(round.status) else {
            break;
        };
        round.status = next;
        round.stage_started_at = Some(deadline);
        round.stage_deadline_at = stage_duration_ms(next, settings).map(|d| deadline + d);
        advanced = true;
    }

    if advanced {
        round.updated_at = now_ms;
        round.upsert(cli).await.map_err(|e| {
            crate::error!("advance_round_if_due upsert failed: {e}");
            FactOrFoldError::StorageFailure
        })?;
    }

    Ok(round)
}

// ── Pure-function tests ───────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> FactOrFoldSettingsResponse {
        FactOrFoldSettingsResponse::default()
    }

    #[test]
    fn next_stage_chain_covers_pr4_stages() {
        assert_eq!(next_stage(RoundStatus::NewsReveal), Some(RoundStatus::Bet));
        assert_eq!(next_stage(RoundStatus::Bet), Some(RoundStatus::Rationale));
        assert_eq!(
            next_stage(RoundStatus::Rationale),
            Some(RoundStatus::Reveal),
        );
        // PR4 stops at Reveal — PR5 extends the chain past it.
        assert_eq!(next_stage(RoundStatus::Reveal), None);
        assert_eq!(next_stage(RoundStatus::Waiting), None);
        assert_eq!(next_stage(RoundStatus::Settled), None);
    }

    #[test]
    fn stage_duration_uses_settings() {
        let s = settings();
        assert_eq!(
            stage_duration_ms(RoundStatus::NewsReveal, &s),
            Some((s.stage_news_reveal_sec as i64) * 1000),
        );
        assert_eq!(
            stage_duration_ms(RoundStatus::Bet, &s),
            Some((s.stage_bet_sec as i64) * 1000),
        );
        assert_eq!(stage_duration_ms(RoundStatus::Waiting, &s), None);
        assert_eq!(stage_duration_ms(RoundStatus::Settled, &s), None);
    }
}
