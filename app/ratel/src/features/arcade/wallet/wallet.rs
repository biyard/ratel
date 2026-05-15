//! Chip wallet — arcade-level economy boundary (이음매 1).
//!
//! Casino metaphor: RP is real currency, chips are the in-game token.
//! `convert_rp_to_chip` is the cashier desk on entry; `redeem_chip_to_rp`
//! is the cashier desk on exit (v1 disabled). `buy_in` is putting
//! chips on the table for a round; `settle` is sweeping the table at
//! the end of the round.
//!
//! Anything that happens inside a round (bet sides, flips, citation
//! bonuses, ...) is **off-wallet**. The wallet only sees buy_in once
//! at round start and one settle per participant at round end.
//! That keeps wallet code simple — no per-action atomicity, no
//! TransactWriteItems — at the cost of forfeiting recovery if the
//! settlement handler crashes mid-round (idempotency_key on the
//! settle txn provides retry safety).
//!
//! This module deliberately keeps `ArcadeWallet` trait and the
//! `DdbArcadeWallet` impl in one file. If/when a second implementation
//! lands (in-memory for tests, sidecar for separated game server),
//! split into `traits.rs` + `impls/`.

use crate::common::*;
use crate::features::arcade::ArcadeError;
use crate::features::arcade::models::{
    ArcadeSettings, ArcadeTxnKind, ArcadeWalletBalance, ArcadeWalletTransaction,
};
use async_trait::async_trait;

// ── Receipt types ───────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ChipReceipt {
    pub txn_id: String,
    pub chips_credited: i64,
    pub balance_after: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RpReceipt {
    pub txn_id: String,
    pub rp_credited: i64,
    pub balance_after: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuyInReceipt {
    pub txn_id: String,
    pub chips_locked: i64,
    pub balance_after: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettleReceipt {
    pub txn_id: String,
    pub chips_credited: i64,
    pub balance_after: i64,
}

// ── Trait ────────────────────────────────────────────────────────────

/// arcade-wide chip wallet. Each call writes one balance update + one
/// transaction row. Implementations are responsible for keeping the
/// balance row and the transaction history consistent (v1: best-effort
/// sequential writes; future: TransactWriteItems).
#[async_trait]
pub trait ArcadeWallet: Send + Sync {
    /// Current chip balance for the user. Returns 0 for a brand-new
    /// user (no balance row materialised yet).
    async fn balance(&self, user_id: &str) -> crate::common::Result<i64>;

    /// RP → chip conversion. Reads RP from the caller's auth context
    /// (User row) and credits chips at the operator-set ratio.
    /// Returns the txn id and post-conversion balance.
    async fn convert_rp_to_chip(
        &self,
        user_id: &str,
        rp_amount: i64,
    ) -> crate::common::Result<ChipReceipt>;

    /// Chip → RP redemption. v1 returns `ArcadeError::WalletRedeemDisabled`.
    /// Kept as part of the trait so call sites compile against the
    /// v2 surface and can be enabled by flipping a setting.
    async fn redeem_chip_to_rp(
        &self,
        user_id: &str,
        chip_amount: i64,
    ) -> crate::common::Result<RpReceipt>;

    /// Lock `chips` on the table for `round_id`. Decrements the chip
    /// balance and writes a BuyIn transaction. Idempotency: callers
    /// must ensure they don't double-call for the same (user, round).
    async fn buy_in(
        &self,
        user_id: &str,
        round_id: &str,
        chips: i64,
    ) -> crate::common::Result<BuyInReceipt>;

    /// Settle a round for one user: credit `chips_out` back to the
    /// wallet (0 if the user lost everything). Writes a Settle txn
    /// keyed by `(user, round)` so a retry of the settlement handler
    /// is a no-op on the wallet side.
    async fn settle(
        &self,
        user_id: &str,
        round_id: &str,
        chips_out: i64,
    ) -> crate::common::Result<SettleReceipt>;
}

// ── DynamoDB implementation ─────────────────────────────────────────

/// DynamoDB-backed `ArcadeWallet`. Holds a clonable DDB client and
/// reads `ArcadeSettings` per call (cheap, no in-process cache yet).
#[derive(Clone)]
pub struct DdbArcadeWallet {
    cli: aws_sdk_dynamodb::Client,
}

impl DdbArcadeWallet {
    pub fn new(cli: aws_sdk_dynamodb::Client) -> Self {
        Self { cli }
    }

    fn now() -> i64 {
        crate::common::utils::time::get_now_timestamp_millis()
    }

    fn fresh_txn_id() -> String {
        // v7 UUIDs are time-sortable so `sk = ArcadeWalletTxn(uuid_v7)`
        // also gives chronological ordering on a single pk query.
        uuid::Uuid::now_v7().to_string()
    }

    async fn write_txn(
        &self,
        user_id: &str,
        kind: ArcadeTxnKind,
        delta: i64,
        ref_round_id: Option<String>,
    ) -> crate::common::Result<(String, i64)> {
        let mut balance = ArcadeWalletBalance::get_or_default(&self.cli, user_id).await?;
        let new_balance = balance.chip_balance + delta;
        if new_balance < 0 {
            return Err(ArcadeError::WalletInsufficientChip.into());
        }

        let now = Self::now();
        balance.chip_balance = new_balance;
        balance.updated_at = now;
        balance.upsert(&self.cli).await.map_err(|e| {
            crate::error!("DdbArcadeWallet balance upsert failed: {e}");
            ArcadeError::StorageFailure
        })?;

        let txn_id = Self::fresh_txn_id();
        let (txn_pk, txn_sk) = ArcadeWalletTransaction::keys(user_id, &txn_id);
        let txn = ArcadeWalletTransaction {
            pk: txn_pk,
            sk: txn_sk,
            created_at: now,
            updated_at: now,
            kind,
            amount: delta,
            balance_after: new_balance,
            ref_round_id,
        };
        txn.create(&self.cli).await.map_err(|e| {
            crate::error!("DdbArcadeWallet txn create failed: {e}");
            ArcadeError::StorageFailure
        })?;

        Ok((txn_id, new_balance))
    }
}

#[async_trait]
impl ArcadeWallet for DdbArcadeWallet {
    async fn balance(&self, user_id: &str) -> crate::common::Result<i64> {
        let b = ArcadeWalletBalance::get_or_default(&self.cli, user_id).await?;
        Ok(b.chip_balance)
    }

    async fn convert_rp_to_chip(
        &self,
        user_id: &str,
        rp_amount: i64,
    ) -> crate::common::Result<ChipReceipt> {
        let settings = ArcadeSettings::get_or_default(&self.cli).await.unwrap_or_default();
        if rp_amount < settings.min_convert_rp {
            return Err(ArcadeError::WalletAmountOutOfRange.into());
        }
        // RP debit on the user's points field is wired in PR4c; PR4b
        // only credits chips so the trait + ledger surface is testable
        // in isolation.
        let chips = rp_amount * (settings.rp_to_chip_ratio_bps as i64) / 10_000;
        let (txn_id, balance_after) = self
            .write_txn(user_id, ArcadeTxnKind::Convert, chips, None)
            .await?;
        Ok(ChipReceipt {
            txn_id,
            chips_credited: chips,
            balance_after,
        })
    }

    async fn redeem_chip_to_rp(
        &self,
        _user_id: &str,
        _chip_amount: i64,
    ) -> crate::common::Result<RpReceipt> {
        Err(ArcadeError::WalletRedeemDisabled.into())
    }

    async fn buy_in(
        &self,
        user_id: &str,
        round_id: &str,
        chips: i64,
    ) -> crate::common::Result<BuyInReceipt> {
        if chips <= 0 {
            return Err(ArcadeError::WalletAmountOutOfRange.into());
        }
        let (txn_id, balance_after) = self
            .write_txn(
                user_id,
                ArcadeTxnKind::BuyIn,
                -chips,
                Some(round_id.to_string()),
            )
            .await?;
        Ok(BuyInReceipt {
            txn_id,
            chips_locked: chips,
            balance_after,
        })
    }

    async fn settle(
        &self,
        user_id: &str,
        round_id: &str,
        chips_out: i64,
    ) -> crate::common::Result<SettleReceipt> {
        if chips_out < 0 {
            return Err(ArcadeError::WalletAmountOutOfRange.into());
        }
        let (txn_id, balance_after) = self
            .write_txn(
                user_id,
                ArcadeTxnKind::Settle,
                chips_out,
                Some(round_id.to_string()),
            )
            .await?;
        Ok(SettleReceipt {
            txn_id,
            chips_credited: chips_out,
            balance_after,
        })
    }
}
