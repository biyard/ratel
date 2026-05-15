//! Stage state machine for *Fact or Fold* rounds.
//!
//! PR4d delegates the actual chain logic to
//! [`crate::features::arcade::services::StageScheduler`] — this file
//! now just (a) implements the trait for FOF's `RoundStatus`
//! and `FactOrFoldSettingsResponse`, and (b) adapts the generic
//! `StageClock` ↔ `FactFoldRound` representation so existing call
//! sites in lobby/round_play continue to compile against the same
//! signatures.
//!
//! The chain is one-way and monotonic:
//!
//!   `NewsReveal` → `Bet` → `Rationale` → `Reveal` → `Debate` → (Settlement, PR6)
//!
//! PR5 extends the chain to `Debate` (live-debate stage with the
//! last-10s flip slot). The round sits in `Debate` once its
//! deadline passes here — the `Debate → Settlement` hand-off
//! lands in PR6 along with the EventBridge settlement trigger.
//!
//! ### Advancement strategy
//!
//! v1 uses two complementary signals — *both* eventually route
//! through [`advance_round_if_due`]:
//! 1. **Client `/tick` (primary):** when the per-stage countdown
//!    hits zero the client calls `POST /rounds/{id}/tick`, which
//!    runs the advance helper and broadcasts the result. See
//!    `controllers::round_play::tick_handler`.
//! 2. **Lazy advance (safety net):** every other read/write on a
//!    round (`GET /rounds/{id}`, `POST /bets`, `POST /rationale`,
//!    etc.) also runs the helper so a stale client still observes
//!    the correct stage.

use crate::common::Result;
use crate::features::arcade::games::fact_or_fold::models::FactFoldRound;
use crate::features::arcade::games::fact_or_fold::types::{
    FactOrFoldError, FactOrFoldSettingsResponse, RoundStatus,
};
use crate::features::arcade::services::{self as arcade_services, StageClock, StageScheduler};

// ── arcade::StageScheduler impl for FOF ─────────────────────────────

/// FOF's [`StageScheduler`] implementation. Pure: no I/O, no
/// allocation. Drives the arcade-level `advance_if_due` walker.
pub struct FactFoldStageScheduler;

impl StageScheduler for FactFoldStageScheduler {
    type Stage = RoundStatus;
    type Settings = FactOrFoldSettingsResponse;

    fn next_stage(current: RoundStatus) -> Option<RoundStatus> {
        match current {
            RoundStatus::NewsReveal => Some(RoundStatus::Bet),
            RoundStatus::Bet => Some(RoundStatus::Rationale),
            RoundStatus::Rationale => Some(RoundStatus::Reveal),
            RoundStatus::Reveal => Some(RoundStatus::Debate),
            // Debate → Settlement is PR6 (EventBridge trigger); the
            // round sits in `Debate` here once its deadline elapses.
            _ => None,
        }
    }

    fn stage_duration_ms(stage: RoundStatus, s: &FactOrFoldSettingsResponse) -> Option<i64> {
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
}

// ── Back-compat free functions ──────────────────────────────────────
//
// Pre-PR4d this file exposed plain `next_stage` and `stage_duration_ms`
// helpers. Keeping them as one-line shims means PR4d is a pure
// "swap the engine" change for callers — no signature churn.

pub fn next_stage(current: RoundStatus) -> Option<RoundStatus> {
    <FactFoldStageScheduler as StageScheduler>::next_stage(current)
}

pub fn stage_duration_ms(stage: RoundStatus, s: &FactOrFoldSettingsResponse) -> Option<i64> {
    <FactFoldStageScheduler as StageScheduler>::stage_duration_ms(stage, s)
}

// ── Adapters (FactFoldRound ↔ StageClock) ───────────────────────────

fn clock_from_round(round: &FactFoldRound) -> StageClock<RoundStatus> {
    StageClock {
        stage: round.status,
        stage_started_at: round.stage_started_at,
        stage_deadline_at: round.stage_deadline_at,
    }
}

fn write_clock_back(round: &mut FactFoldRound, clock: StageClock<RoundStatus>) {
    round.status = clock.stage;
    round.stage_started_at = clock.stage_started_at;
    round.stage_deadline_at = clock.stage_deadline_at;
}

/// Stamp the initial stage clock on the round. Called by the lobby
/// matching service exactly once, on the Waiting → NewsReveal
/// transition.
pub fn stamp_initial_stage(
    round: &mut FactFoldRound,
    settings: &FactOrFoldSettingsResponse,
    now_ms: i64,
) {
    let mut clock = clock_from_round(round);
    arcade_services::stamp_initial_stage::<FactFoldStageScheduler>(&mut clock, settings, now_ms);
    write_clock_back(round, clock);
}

/// Walk the round forward through every stage whose deadline has
/// already passed at `now_ms`. Persists at most once at the end.
pub async fn advance_round_if_due(
    cli: &aws_sdk_dynamodb::Client,
    mut round: FactFoldRound,
    settings: &FactOrFoldSettingsResponse,
    now_ms: i64,
) -> Result<FactFoldRound> {
    let mut clock = clock_from_round(&round);
    let outcome = arcade_services::advance_if_due::<FactFoldStageScheduler>(
        &mut clock, settings, now_ms,
    );
    if outcome.persisted_needed {
        write_clock_back(&mut round, clock);
        round.updated_at = now_ms;
        round.upsert(cli).await.map_err(|e| {
            crate::error!("advance_round_if_due upsert failed: {e}");
            FactOrFoldError::StorageFailure
        })?;
    }
    Ok(round)
}

// ── FOF-specific tests ──────────────────────────────────────────────
//
// Generic walker tests live alongside the arcade impl in
// `arcade::services::stage_scheduler::tests`. Here we only verify
// the FOF-specific mapping (which RoundStatus pairs are valid
// transitions, which durations come from which setting field).

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> FactOrFoldSettingsResponse {
        FactOrFoldSettingsResponse::default()
    }

    #[test]
    fn next_stage_chain_covers_pr5_stages() {
        assert_eq!(next_stage(RoundStatus::NewsReveal), Some(RoundStatus::Bet));
        assert_eq!(next_stage(RoundStatus::Bet), Some(RoundStatus::Rationale));
        assert_eq!(
            next_stage(RoundStatus::Rationale),
            Some(RoundStatus::Reveal),
        );
        assert_eq!(next_stage(RoundStatus::Reveal), Some(RoundStatus::Debate));
        // PR5 stops at Debate — PR6 wires Debate → Settlement via
        // the EventBridge settlement trigger.
        assert_eq!(next_stage(RoundStatus::Debate), None);
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
