//! Stage scheduler — arcade-level generic over each game's stage
//! enum (이음매 3).
//!
//! v1 trigger model (design doc § A6): client sends `POST /tick`
//! when its countdown hits zero; server calls [`advance_if_due`] to
//! check the wall-clock against the round's `stage_deadline_at` and
//! ratchet forward. The same helper also runs on every read/write
//! path of the round as a safety net (lazy advance).
//!
//! Each game implements [`StageScheduler`] for its own stage enum.
//! Generic `advance_if_due` consumes that impl + a mutable
//! `StageClock` snapshot, walks the chain forward in pure-function
//! style, and returns whether anything changed. **No I/O** — the
//! caller persists the resulting `StageClock` back to its store.

/// Mutable stage clock view a game gives the scheduler. Decoupled
/// from any concrete DDB entity so the scheduler can run against
/// in-memory snapshots in tests and against a `FactFoldRound` row
/// in prod.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageClock<S> {
    pub stage: S,
    /// Millis-since-epoch when the current stage began. `None` while
    /// the game is in its lobby/waiting state.
    pub stage_started_at: Option<i64>,
    /// Millis-since-epoch when the current stage auto-advances.
    /// `None` for stages without a deadline (lobby, terminal).
    pub stage_deadline_at: Option<i64>,
}

/// Outcome of [`advance_if_due`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvanceOutcome<S> {
    /// How many stages were crossed. `0` means nothing changed.
    pub stages_advanced: usize,
    /// Final stage after the walk.
    pub final_stage: S,
    /// `true` iff the caller should persist `StageClock` back.
    pub persisted_needed: bool,
}

/// One stage scheduler per game. Implementations are *pure* —
/// `next_stage` and `stage_duration_ms` must not touch DDB. Side
/// effects (broadcast, settlement trigger, etc.) belong elsewhere.
pub trait StageScheduler {
    /// Game's stage enum.
    type Stage: Copy + Eq;
    /// Game's settings type (carries per-stage duration knobs).
    type Settings;

    /// Successor stage in the auto-progression chain. `None` for
    /// terminal stages or stages the game intentionally doesn't
    /// auto-advance past (e.g. FOF stopping at `Reveal` in PR4).
    fn next_stage(current: Self::Stage) -> Option<Self::Stage>;

    /// Duration of `stage` in milliseconds. `None` for stages with
    /// no fixed deadline (lobby, event-driven terminal stages).
    fn stage_duration_ms(stage: Self::Stage, settings: &Self::Settings) -> Option<i64>;
}

/// Stamp the initial stage clock. Used at the lobby→first-stage
/// transition: sets `stage_started_at = now_ms` and computes
/// `stage_deadline_at` from the scheduler's duration.
pub fn stamp_initial_stage<S: StageScheduler>(
    clock: &mut StageClock<S::Stage>,
    settings: &S::Settings,
    now_ms: i64,
) {
    clock.stage_started_at = Some(now_ms);
    clock.stage_deadline_at = S::stage_duration_ms(clock.stage, settings).map(|d| now_ms + d);
}

/// Walk the clock forward through every stage whose deadline has
/// passed at `now_ms`. Pure: writes back into `clock`, no I/O.
///
/// Anchors each new deadline to the *previous* deadline (not
/// `now_ms`) so total round length stays bounded by the sum of
/// stage durations even if the caller is late.
pub fn advance_if_due<S: StageScheduler>(
    clock: &mut StageClock<S::Stage>,
    settings: &S::Settings,
    now_ms: i64,
) -> AdvanceOutcome<S::Stage> {
    let mut advanced: usize = 0;
    loop {
        let Some(deadline) = clock.stage_deadline_at else {
            break;
        };
        if now_ms < deadline {
            break;
        }
        let Some(next) = S::next_stage(clock.stage) else {
            break;
        };
        clock.stage = next;
        clock.stage_started_at = Some(deadline);
        clock.stage_deadline_at = S::stage_duration_ms(next, settings).map(|d| deadline + d);
        advanced += 1;
    }
    AdvanceOutcome {
        stages_advanced: advanced,
        final_stage: clock.stage,
        persisted_needed: advanced > 0,
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum FakeStage {
        A,
        B,
        C,
        Terminal,
    }

    struct FakeSettings {
        a_ms: i64,
        b_ms: i64,
        c_ms: i64,
    }

    struct FakeScheduler;
    impl StageScheduler for FakeScheduler {
        type Stage = FakeStage;
        type Settings = FakeSettings;
        fn next_stage(current: FakeStage) -> Option<FakeStage> {
            match current {
                FakeStage::A => Some(FakeStage::B),
                FakeStage::B => Some(FakeStage::C),
                FakeStage::C => Some(FakeStage::Terminal),
                FakeStage::Terminal => None,
            }
        }
        fn stage_duration_ms(stage: FakeStage, s: &FakeSettings) -> Option<i64> {
            match stage {
                FakeStage::A => Some(s.a_ms),
                FakeStage::B => Some(s.b_ms),
                FakeStage::C => Some(s.c_ms),
                FakeStage::Terminal => None,
            }
        }
    }

    fn settings() -> FakeSettings {
        FakeSettings {
            a_ms: 1_000,
            b_ms: 2_000,
            c_ms: 3_000,
        }
    }

    #[test]
    fn stamp_initial_stage_sets_both_fields() {
        let mut c = StageClock {
            stage: FakeStage::A,
            stage_started_at: None,
            stage_deadline_at: None,
        };
        stamp_initial_stage::<FakeScheduler>(&mut c, &settings(), 1_000);
        assert_eq!(c.stage_started_at, Some(1_000));
        assert_eq!(c.stage_deadline_at, Some(2_000)); // 1_000 + 1_000
    }

    #[test]
    fn advance_noop_before_deadline() {
        let mut c = StageClock {
            stage: FakeStage::A,
            stage_started_at: Some(0),
            stage_deadline_at: Some(1_000),
        };
        let out = advance_if_due::<FakeScheduler>(&mut c, &settings(), 999);
        assert_eq!(out.stages_advanced, 0);
        assert_eq!(c.stage, FakeStage::A);
    }

    #[test]
    fn advance_one_stage_at_a_time() {
        let mut c = StageClock {
            stage: FakeStage::A,
            stage_started_at: Some(0),
            stage_deadline_at: Some(1_000),
        };
        let out = advance_if_due::<FakeScheduler>(&mut c, &settings(), 1_500);
        assert_eq!(out.stages_advanced, 1);
        assert_eq!(c.stage, FakeStage::B);
        // anchored to previous deadline (1000), not now (1500)
        assert_eq!(c.stage_started_at, Some(1_000));
        assert_eq!(c.stage_deadline_at, Some(3_000)); // 1000 + 2000
    }

    #[test]
    fn advance_walks_multiple_stages() {
        let mut c = StageClock {
            stage: FakeStage::A,
            stage_started_at: Some(0),
            stage_deadline_at: Some(1_000),
        };
        // 1_000 (A→B at 1000) → 3_000 (B→C at 3000) → 6_000 (C→Terminal at 6000)
        let out = advance_if_due::<FakeScheduler>(&mut c, &settings(), 10_000);
        assert_eq!(out.stages_advanced, 3);
        assert_eq!(c.stage, FakeStage::Terminal);
        // terminal has no duration, so deadline is None
        assert_eq!(c.stage_deadline_at, None);
    }

    #[test]
    fn advance_stops_at_terminal_even_if_past() {
        let mut c = StageClock {
            stage: FakeStage::Terminal,
            stage_started_at: Some(0),
            stage_deadline_at: None,
        };
        let out = advance_if_due::<FakeScheduler>(&mut c, &settings(), 999_999);
        assert_eq!(out.stages_advanced, 0);
        assert_eq!(c.stage, FakeStage::Terminal);
    }
}
