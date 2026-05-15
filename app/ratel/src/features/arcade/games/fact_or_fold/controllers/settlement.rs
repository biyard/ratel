//! Settlement handler (PR6 step 2).
//!
//! Surface:
//!   POST /api/fact-or-fold/admin/rounds/{round_id}/settle
//!     — manual operator-triggered settlement (admin-gated). Step 3
//!       adds the EventBridge auto-trigger that calls
//!       `settle_round_internal` on every Debate-stage round whose
//!       deadline elapsed.
//!
//! ### Flow
//!
//! 1. Load round; bail if already `Settled` (idempotent — caller
//!    can retry without doubling pay-outs).
//! 2. Load headline (verdict), bets, rationales, participants,
//!    arcade settings, FOF settings (for chip multipliers).
//! 3. Run the pure `settle_round` formula.
//! 4. For each `SettlementOutcome`:
//!    a. `FactFoldSettlement::create` (server defaults to
//!       `create_if_not_exists` semantics via the DynamoEntity
//!       `create` path; a re-run on an existing row errors and
//!       we treat that as "already settled, skip").
//!    b. `ArcadeWallet::settle(user, round, chips_out)` — credits
//!       chips back to the wallet. `wallet.settle` writes its own
//!       ledger row keyed off a fresh txn id; retry safety here
//!       depends on the settlement row's "already exists" gate
//!       above (we skip the whole user when the settlement row
//!       exists).
//!    c. `FactFoldUserStats` upsert — `total_rounds += 1`,
//!       `correct_count += 1 if won`, `lifetime_delta_chips +=
//!       chips_out - buy_in`, `last_played_at = now`. The buy_in
//!       amount comes from `arcade_settings.default_buy_in_chips`
//!       (per A4: per-round buy-in is uniform in v1).
//! 5. Round → `Settled`, `settled_at = now`.

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[cfg(feature = "server")]
use crate::common::models::auth::AdminUser;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{
    FactFoldBet, FactFoldHeadline, FactFoldLeaderboardEntry, FactFoldParticipant,
    FactFoldRationale, FactFoldRound, FactFoldSettings, FactFoldSettlement, FactFoldUserStats,
};
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::services::{
    settle_round, SettleRoundInput, SettlementOutcome,
};
#[cfg(feature = "server")]
use crate::features::arcade::models::ArcadeSettings;
#[cfg(feature = "server")]
use crate::features::arcade::wallet::{ArcadeWallet, DdbArcadeWallet};

// ── DTO ─────────────────────────────────────────────────────────────

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SettleRoundResponse {
    pub round_id: String,
    /// Per-user outcomes. Already settled rounds return the
    /// previously-persisted breakdown.
    pub outcomes: Vec<SettlementBreakdown>,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SettlementBreakdown {
    pub user_pk: String,
    pub stake: i64,
    pub won: bool,
    pub base_refund: i64,
    pub correct_bonus: i64,
    pub pool_share: i64,
    pub influence_bonus: i64,
    pub insider_bonus: i64,
    pub chips_out: i64,
}

#[cfg(feature = "server")]
impl From<&SettlementOutcome> for SettlementBreakdown {
    fn from(o: &SettlementOutcome) -> Self {
        Self {
            user_pk: format!("USER#{}", o.user_id),
            stake: o.stake,
            won: o.won,
            base_refund: o.base_refund,
            correct_bonus: o.correct_bonus,
            pool_share: o.pool_share,
            influence_bonus: o.influence_bonus,
            insider_bonus: o.insider_bonus,
            chips_out: o.chips_out,
        }
    }
}

// ── Public entry point ──────────────────────────────────────────────

/// Idempotent settle-a-round side-effect bundle. Called by:
///   - The manual admin endpoint below.
///   - The EventBridge settlement trigger (PR6 step 3).
///
/// Safe to call repeatedly on the same round — the first call
/// flips `round.status = Settled`; subsequent calls return the
/// already-persisted breakdown without re-running the wallet
/// payout (the per-user settlement row's existence is the gate).
#[cfg(feature = "server")]
pub async fn settle_round_internal(
    cli: &aws_sdk_dynamodb::Client,
    round_id: &str,
) -> Result<SettleRoundResponse> {
    let (round_pk, round_sk) = FactFoldRound::keys(round_id);
    let mut round = FactFoldRound::get(cli, &round_pk, Some(round_sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("settle_round_internal round read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    // Idempotency: if the round is already Settled, just return
    // the already-persisted breakdown.
    if matches!(round.status, RoundStatus::Settled) {
        let breakdowns = load_existing_breakdowns(cli, &round_pk).await?;
        return Ok(SettleRoundResponse {
            round_id: round_id.to_string(),
            outcomes: breakdowns,
        });
    }

    // Load all the round-scoped rows. v1 fits comfortably in a
    // single pk query (4 players, ≤8 rows total — round + 4
    // participants + 4 bets + ≤4 rationales + settlement targets).
    let headline_pk = FactFoldHeadline::anchor_pk();
    let headline_sk: EntityType =
        crate::FactFoldHeadlineEntityType(round.headline_id.clone()).into();
    let headline = FactFoldHeadline::get(cli, &headline_pk, Some(headline_sk))
        .await
        .map_err(|e| {
            crate::error!("settle_round_internal headline read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;

    // Per-participant entities.
    let opts_p = FactFoldParticipant::opt()
        .sk("FACT_FOLD_PARTICIPANT".to_string())
        .limit(50);
    let (participants, _) = FactFoldParticipant::query(cli, round_pk.clone(), opts_p)
        .await
        .map_err(|e| {
            crate::error!("settle_round_internal participants query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let opts_b = FactFoldBet::opt().sk("FACT_FOLD_BET".to_string()).limit(50);
    let (bets, _) = FactFoldBet::query(cli, round_pk.clone(), opts_b)
        .await
        .map_err(|e| {
            crate::error!("settle_round_internal bets query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let opts_r = FactFoldRationale::opt()
        .sk("FACT_FOLD_RATIONALE".to_string())
        .limit(50);
    let (rationales, _) = FactFoldRationale::query(cli, round_pk.clone(), opts_r)
        .await
        .map_err(|e| {
            crate::error!("settle_round_internal rationales query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let fof_settings = FactFoldSettings::get_or_default(cli)
        .await
        .unwrap_or_default();
    let arcade_settings = ArcadeSettings::get_or_default(cli).await.unwrap_or_default();
    let buy_in = arcade_settings.default_buy_in_chips;

    let outcomes = settle_round(SettleRoundInput {
        verdict: headline.verdict,
        bets: &bets,
        rationales: &rationales,
        participants: &participants,
        settings: &fof_settings,
    });

    let wallet = DdbArcadeWallet::new(cli.clone());
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut breakdowns: Vec<SettlementBreakdown> = Vec::with_capacity(outcomes.len());

    for o in outcomes.iter() {
        // Per-user settlement row. `create` errors if a row already
        // exists — we treat that as "this user already settled,
        // skip the wallet + stats steps to avoid double credit".
        let (settle_pk, settle_sk) = FactFoldSettlement::keys(round_id, &o.user_id);
        let row = FactFoldSettlement {
            pk: settle_pk,
            sk: settle_sk,
            created_at: now,
            updated_at: now,
            user_pk: Partition::User(o.user_id.clone()),
            idempotency_key: FactFoldSettlement::idempotency_key_for(round_id, &o.user_id),
            base_refund: o.base_refund,
            correct_bonus: o.correct_bonus,
            pool_share: o.pool_share,
            influence_bonus: o.influence_bonus,
            insider_bonus: o.insider_bonus,
            chips_out: o.chips_out,
        };
        let inserted = row.create(cli).await.is_ok();
        breakdowns.push(SettlementBreakdown::from(o));

        if !inserted {
            // Settlement row already existed — skip wallet + stats.
            // Either a previous run for this user succeeded (so we
            // don't double credit) or a concurrent run is in
            // progress (the other run is the source of truth).
            continue;
        }

        // Credit chips back to the wallet.
        if let Err(e) = wallet.settle(&o.user_id, round_id, o.chips_out).await {
            crate::error!(
                "settle_round_internal wallet.settle failed for {}: {e}",
                o.user_id
            );
        }

        // Update lifetime stats.
        let mut stats = FactFoldUserStats::get_or_default(cli, &o.user_id).await?;
        let prev_accuracy_bps = compute_accuracy_bps(stats.correct_count, stats.total_rounds);
        stats.total_rounds += 1;
        if o.won {
            stats.correct_count += 1;
        }
        stats.lifetime_delta_chips += o.chips_out - buy_in;
        stats.last_played_at = now;
        stats.updated_at = now;
        if let Err(e) = stats.upsert(cli).await {
            crate::error!(
                "settle_round_internal user_stats upsert failed for {}: {e}",
                o.user_id
            );
        }

        // Mirror to the leaderboard anchor: delete the previous
        // accuracy-keyed row (if any), then write the new one.
        // The leaderboard is sk-DESC at `Partition::FactFoldLeaderboard`
        // so a query returns top-accuracy users first.
        let new_accuracy_bps = compute_accuracy_bps(stats.correct_count, stats.total_rounds);
        if stats.total_rounds > 1 {
            let (prev_pk, prev_sk) =
                FactFoldLeaderboardEntry::keys(prev_accuracy_bps, &o.user_id);
            let _ = FactFoldLeaderboardEntry::delete(cli, &prev_pk, Some(prev_sk)).await;
        }
        let (lb_pk, lb_sk) = FactFoldLeaderboardEntry::keys(new_accuracy_bps, &o.user_id);
        let entry = FactFoldLeaderboardEntry {
            pk: lb_pk,
            sk: lb_sk,
            created_at: stats.created_at,
            updated_at: now,
            user_pk: Partition::User(o.user_id.clone()),
            accuracy_bps: new_accuracy_bps,
            total_rounds: stats.total_rounds,
            correct_count: stats.correct_count,
            lifetime_delta_chips: stats.lifetime_delta_chips,
            last_played_at: now,
        };
        if let Err(e) = entry.upsert(cli).await {
            crate::error!(
                "settle_round_internal leaderboard upsert failed for {}: {e}",
                o.user_id
            );
        }
    }

    // Flip the round terminal state. Even if some per-user steps
    // failed, the round is settled — the audit trail lives on the
    // settlement rows that exist (or didn't).
    round.status = RoundStatus::Settled;
    round.settled_at = Some(now);
    round.updated_at = now;
    round.upsert(cli).await.map_err(|e| {
        crate::error!("settle_round_internal round upsert failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(SettleRoundResponse {
        round_id: round_id.to_string(),
        outcomes: breakdowns,
    })
}

/// `correct_count / total_rounds`, in basis points. A 0-round
/// player is treated as 0% accuracy for ranking purposes.
#[cfg(feature = "server")]
pub fn compute_accuracy_bps(correct: i64, total: i64) -> i32 {
    if total <= 0 {
        return 0;
    }
    let bps = (correct * 10_000) / total;
    bps.clamp(0, 10_000) as i32
}

#[cfg(feature = "server")]
async fn load_existing_breakdowns(
    cli: &aws_sdk_dynamodb::Client,
    round_pk: &Partition,
) -> Result<Vec<SettlementBreakdown>> {
    let opts = FactFoldSettlement::opt()
        .sk("FACT_FOLD_SETTLEMENT".to_string())
        .limit(50);
    let (rows, _) = FactFoldSettlement::query(cli, round_pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("load_existing_breakdowns query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;
    Ok(rows
        .into_iter()
        .map(|r| {
            // The persisted row carries the same shape as the live
            // outcome; we just project it.
            let won = r.base_refund > 0;
            SettlementBreakdown {
                user_pk: r.user_pk.to_string(),
                stake: 0, // not persisted on the settlement row
                won,
                base_refund: r.base_refund,
                correct_bonus: r.correct_bonus,
                pool_share: r.pool_share,
                influence_bonus: r.influence_bonus,
                insider_bonus: r.insider_bonus,
                chips_out: r.chips_out,
            }
        })
        .collect())
}

// ── Admin endpoint ──────────────────────────────────────────────────

#[post(
    "/api/fact-or-fold/admin/rounds/{round_id}/settle",
    _user: AdminUser
)]
pub async fn admin_settle_round_handler(
    round_id: FactFoldRoundEntityType,
) -> Result<SettleRoundResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    settle_round_internal(cli, &round_id.0).await
}
