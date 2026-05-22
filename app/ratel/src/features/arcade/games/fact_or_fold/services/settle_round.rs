//! Pure settlement formula — roadmap §FR-28~30. No I/O: takes
//! input snapshots, returns one [`SettlementOutcome`] per
//! participant. The caller (`controllers::settlement` in step 2)
//! persists the `FactFoldSettlement` rows, calls
//! `ArcadeWallet::settle`, and updates `FactFoldUserStats`.
//!
//! Keeping the math pure means:
//! - all combinatorics live in one file with unit tests
//! - the handler can be replayed (idempotency via the settlement
//!   row's create-if-not-exists) without re-deriving anything
//! - PR7's leaderboard can re-use the per-user breakdown shape
//!
//! All amounts are *chips*. The `bet.amount_rp` field name is
//! historical (PR1 wrote it before the chip metaphor landed) —
//! the value is in chip units after the wallet refactor.

use crate::features::arcade::games::fact_or_fold::models::{
    FactFoldBet, FactFoldParticipant, FactFoldRationale,
};
use crate::features::arcade::games::fact_or_fold::types::{
    BetSide, FactOrFoldSettingsResponse, Verdict,
};

/// Snapshot of one user's effective side after the §FR-16 flip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalSide {
    Real,
    Fake,
}

impl FinalSide {
    pub fn from_verdict(v: Verdict) -> Self {
        match v {
            Verdict::Real => FinalSide::Real,
            Verdict::Fake => FinalSide::Fake,
        }
    }
    pub fn from_bet_side(s: BetSide) -> Self {
        match s {
            BetSide::Real => FinalSide::Real,
            BetSide::Fake => FinalSide::Fake,
        }
    }
    pub fn from_bet(b: &FactFoldBet) -> Self {
        FinalSide::from_bet_side(b.flipped_to.unwrap_or(b.side))
    }
}

/// Per-user settlement breakdown. The handler persists this as a
/// `FactFoldSettlement` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementOutcome {
    pub user_id: String,
    pub stake: i64,
    pub final_side: FinalSide,
    pub won: bool,

    pub base_refund: i64,
    pub correct_bonus: i64,
    pub pool_share: i64,
    pub influence_bonus: i64,
    pub insider_bonus: i64,

    pub chips_out: i64,
}

impl SettlementOutcome {
    fn empty(user_id: String, stake: i64, final_side: FinalSide, won: bool) -> Self {
        Self {
            user_id,
            stake,
            final_side,
            won,
            base_refund: 0,
            correct_bonus: 0,
            pool_share: 0,
            influence_bonus: 0,
            insider_bonus: 0,
            chips_out: 0,
        }
    }

    fn recompute_chips_out(&mut self) {
        self.chips_out = self.base_refund
            + self.correct_bonus
            + self.pool_share
            + self.influence_bonus
            + self.insider_bonus;
    }
}

/// Inputs to [`settle_round`]. Caller pre-loads everything; this
/// function does no I/O.
pub struct SettleRoundInput<'a> {
    pub verdict: Verdict,
    pub bets: &'a [FactFoldBet],
    pub rationales: &'a [FactFoldRationale],
    pub participants: &'a [FactFoldParticipant],
    pub settings: &'a FactOrFoldSettingsResponse,
}

fn user_id_of(pk: &crate::common::types::Partition) -> String {
    pk.to_string()
        .strip_prefix("USER#")
        .unwrap_or(&pk.to_string())
        .to_string()
}

/// Run the §FR-28~30 formula over a round's snapshot. Returns one
/// [`SettlementOutcome`] per bet. Forfeited / no-bet participants
/// are simply absent from the result — the caller knows they
/// don't get a settlement row.
pub fn settle_round(input: SettleRoundInput<'_>) -> Vec<SettlementOutcome> {
    let truth = FinalSide::from_verdict(input.verdict);

    // Pre-compute final-side + winner classification for every bet.
    let mut outcomes: Vec<SettlementOutcome> = input
        .bets
        .iter()
        .map(|b| {
            let final_side = FinalSide::from_bet(b);
            let won = final_side == truth;
            SettlementOutcome::empty(user_id_of(&b.user_pk), b.amount_rp, final_side, won)
        })
        .collect();

    let winner_stake_sum: i64 = outcomes.iter().filter(|o| o.won).map(|o| o.stake).sum();
    let loser_stake_sum: i64 = outcomes.iter().filter(|o| !o.won).map(|o| o.stake).sum();

    // §FR-28: winners get refund + correct-side bonus +
    // proportional share of the loser pool.
    let correct_bonus_bps = input
        .settings
        .correct_side_multiplier_bps
        .saturating_sub(10_000) as i64;
    for o in outcomes.iter_mut().filter(|o| o.won) {
        o.base_refund = o.stake;
        o.correct_bonus = o.stake * correct_bonus_bps / 10_000;
        // Proportional pool share. Guard against div-by-zero when
        // all participants happen to land on the same (winning)
        // side and there is no loser pool.
        if winner_stake_sum > 0 {
            o.pool_share = loser_stake_sum * o.stake / winner_stake_sum;
        }
    }

    // §FR-29: influence bonus. For each winning *flipper* whose
    // cite points at a real participant + that participant
    // submitted a rationale, the cited user gets
    // `flipper.stake × influence_bonus_bps / 10_000`.
    let rationale_user_ids: std::collections::HashSet<String> = input
        .rationales
        .iter()
        .map(|r| user_id_of(&r.user_pk))
        .collect();
    let influence_bps = input.settings.influence_bonus_bps as i64;
    for (idx, b) in input.bets.iter().enumerate() {
        if !outcomes[idx].won {
            continue;
        }
        let Some(_flipped_to) = b.flipped_to else {
            continue;
        };
        let Some(cite_pk) = b.flip_cite_user_pk.as_ref() else {
            continue;
        };
        let cite_user_id = user_id_of(cite_pk);
        if !rationale_user_ids.contains(&cite_user_id) {
            continue;
        }
        let bonus = b.amount_rp * influence_bps / 10_000;
        if let Some(target) = outcomes.iter_mut().find(|o| o.user_id == cite_user_id) {
            target.influence_bonus += bonus;
        }
    }

    // §FR-30: insider correct-bet bonus. Find the insider via
    // participants (`is_insider == true`); if their final side
    // matched the truth, credit them `stake × insider_bonus_bps`.
    let insider_user_id: Option<String> = input
        .participants
        .iter()
        .find(|p| p.is_insider)
        .map(|p| user_id_of(&p.user_pk));
    if let Some(uid) = insider_user_id {
        if let Some(insider_outcome) = outcomes.iter_mut().find(|o| o.user_id == uid && o.won) {
            let bps = input.settings.insider_correct_bonus_bps as i64;
            insider_outcome.insider_bonus = insider_outcome.stake * bps / 10_000;
        }
    }

    // Finalize chips_out for every outcome.
    for o in outcomes.iter_mut() {
        o.recompute_chips_out();
    }
    outcomes
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::{EntityType, Partition};

    fn settings() -> FactOrFoldSettingsResponse {
        FactOrFoldSettingsResponse::default() // 16_000 / 5_000 / 3_000
    }

    fn bet(user_id: &str, side: BetSide, amount: i64) -> FactFoldBet {
        FactFoldBet {
            pk: Partition::FactFold("r".into()),
            sk: EntityType::FactFoldBet(user_id.into()),
            created_at: 0,
            updated_at: 0,
            user_pk: Partition::User(user_id.into()),
            side,
            amount_rp: amount,
            locked_at: 0,
            flipped_to: None,
            flip_cite_user_pk: None,
        }
    }

    fn participant(user_id: &str, is_insider: bool) -> FactFoldParticipant {
        FactFoldParticipant {
            pk: Partition::FactFold("r".into()),
            sk: EntityType::FactFoldParticipant(user_id.into()),
            created_at: 0,
            updated_at: 0,
            user_pk: Partition::User(user_id.into()),
            joined_at: 0,
            is_insider,
            last_seen_at: 0,
            forfeited: false,
        }
    }

    fn rationale(user_id: &str) -> FactFoldRationale {
        FactFoldRationale {
            pk: Partition::FactFold("r".into()),
            sk: EntityType::FactFoldRationale(user_id.into()),
            created_at: 0,
            updated_at: 0,
            user_pk: Partition::User(user_id.into()),
            text: "rationale".into(),
            submitted_at: 0,
            essence_eligible: true,
            essence_registered: false,
        }
    }

    #[test]
    fn winner_refund_plus_bonus_plus_pool() {
        // 1 winner staking 100, 1 loser staking 100, no flips, no
        // insider. Defaults: correct_bonus_bps = 16_000 → +60% of
        // own stake; loser pool = 100, all to the one winner.
        let bets = vec![
            bet("a", BetSide::Real, 100), // wins
            bet("b", BetSide::Fake, 100), // loses
        ];
        let participants = vec![participant("a", false), participant("b", false)];
        let outcomes = settle_round(SettleRoundInput {
            verdict: Verdict::Real,
            bets: &bets,
            rationales: &[],
            participants: &participants,
            settings: &settings(),
        });
        let a = outcomes.iter().find(|o| o.user_id == "a").unwrap();
        assert!(a.won);
        assert_eq!(a.base_refund, 100);
        assert_eq!(a.correct_bonus, 60); // 100 × 0.6
        assert_eq!(a.pool_share, 100); // entire loser pool
        assert_eq!(a.chips_out, 260);

        let b = outcomes.iter().find(|o| o.user_id == "b").unwrap();
        assert!(!b.won);
        assert_eq!(b.chips_out, 0);
    }

    #[test]
    fn all_on_winning_side_no_pool() {
        // Everyone bets the truth side. No loser pool. Each winner
        // gets refund + correct_bonus only.
        let bets = vec![bet("a", BetSide::Real, 100), bet("b", BetSide::Real, 200)];
        let participants = vec![participant("a", false), participant("b", false)];
        let outcomes = settle_round(SettleRoundInput {
            verdict: Verdict::Real,
            bets: &bets,
            rationales: &[],
            participants: &participants,
            settings: &settings(),
        });
        let a = outcomes.iter().find(|o| o.user_id == "a").unwrap();
        assert_eq!(a.base_refund, 100);
        assert_eq!(a.correct_bonus, 60);
        assert_eq!(a.pool_share, 0);
        assert_eq!(a.chips_out, 160);
    }

    #[test]
    fn flip_with_cite_credits_cited_user() {
        // a flipped FAKE→REAL citing b's rationale; a wins.
        // b should get +30 (a.stake × 30%) influence_bonus.
        let mut a_bet = bet("a", BetSide::Fake, 100);
        a_bet.flipped_to = Some(BetSide::Real);
        a_bet.flip_cite_user_pk = Some(Partition::User("b".into()));
        let bets = vec![a_bet, bet("b", BetSide::Real, 100), bet("c", BetSide::Fake, 100)];
        let participants = vec![
            participant("a", false),
            participant("b", false),
            participant("c", false),
        ];
        let outcomes = settle_round(SettleRoundInput {
            verdict: Verdict::Real,
            bets: &bets,
            rationales: &[rationale("b")],
            participants: &participants,
            settings: &settings(),
        });
        let b = outcomes.iter().find(|o| o.user_id == "b").unwrap();
        assert_eq!(b.influence_bonus, 30, "b should get a.stake × 0.3");
    }

    #[test]
    fn flip_without_rationale_no_influence_bonus() {
        // Cited user b never submitted a rationale → influence_bonus = 0.
        let mut a_bet = bet("a", BetSide::Fake, 100);
        a_bet.flipped_to = Some(BetSide::Real);
        a_bet.flip_cite_user_pk = Some(Partition::User("b".into()));
        let bets = vec![a_bet, bet("b", BetSide::Real, 100), bet("c", BetSide::Fake, 100)];
        let participants = vec![
            participant("a", false),
            participant("b", false),
            participant("c", false),
        ];
        let outcomes = settle_round(SettleRoundInput {
            verdict: Verdict::Real,
            bets: &bets,
            rationales: &[], // b has no rationale
            participants: &participants,
            settings: &settings(),
        });
        let b = outcomes.iter().find(|o| o.user_id == "b").unwrap();
        assert_eq!(b.influence_bonus, 0);
    }

    #[test]
    fn insider_correct_bet_gets_bonus() {
        // a is the insider, bets REAL, truth is REAL.
        let bets = vec![bet("a", BetSide::Real, 100), bet("b", BetSide::Fake, 100)];
        let participants = vec![participant("a", true), participant("b", false)];
        let outcomes = settle_round(SettleRoundInput {
            verdict: Verdict::Real,
            bets: &bets,
            rationales: &[],
            participants: &participants,
            settings: &settings(),
        });
        let a = outcomes.iter().find(|o| o.user_id == "a").unwrap();
        assert_eq!(a.insider_bonus, 50); // 100 × 0.5
        // chips_out = refund(100) + correct(60) + pool(100) + insider(50)
        assert_eq!(a.chips_out, 310);
    }

    #[test]
    fn insider_wrong_bet_no_insider_bonus() {
        let bets = vec![bet("a", BetSide::Fake, 100), bet("b", BetSide::Real, 100)];
        let participants = vec![participant("a", true), participant("b", false)];
        let outcomes = settle_round(SettleRoundInput {
            verdict: Verdict::Real,
            bets: &bets,
            rationales: &[],
            participants: &participants,
            settings: &settings(),
        });
        let a = outcomes.iter().find(|o| o.user_id == "a").unwrap();
        assert_eq!(a.insider_bonus, 0);
        assert_eq!(a.chips_out, 0);
    }

    #[test]
    fn losers_have_zero_chips_out() {
        // §FR-31 — balance never goes negative; loser just sees 0.
        let bets = vec![bet("a", BetSide::Real, 100), bet("b", BetSide::Fake, 200)];
        let participants = vec![participant("a", false), participant("b", false)];
        let outcomes = settle_round(SettleRoundInput {
            verdict: Verdict::Real,
            bets: &bets,
            rationales: &[],
            participants: &participants,
            settings: &settings(),
        });
        let b = outcomes.iter().find(|o| o.user_id == "b").unwrap();
        assert_eq!(b.chips_out, 0);
        assert!(!b.won);
    }
}
