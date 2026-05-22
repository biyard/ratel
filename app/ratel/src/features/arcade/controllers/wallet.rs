//! Wallet endpoints — the cashier desk for the arcade.
//!
//! Surface:
//!   GET   /api/arcade/wallet                  current chip balance + UI hints
//!   POST  /api/arcade/wallet/convert          RP → chip
//!   POST  /api/arcade/wallet/redeem           chip → RP (v1 disabled)

use crate::common::*;
use crate::features::arcade::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::ArcadeError;
#[cfg(feature = "server")]
use crate::features::arcade::models::ArcadeSettings;
#[cfg(feature = "server")]
use crate::features::arcade::wallet::{ArcadeWallet, DdbArcadeWallet};

#[cfg(feature = "server")]
fn user_inner_id(user: &User) -> String {
    user.pk
        .to_string()
        .strip_prefix("USER#")
        .unwrap_or(&user.pk.to_string())
        .to_string()
}

// ── GET /api/arcade/wallet ───────────────────────────────────────────

#[get("/api/arcade/wallet", user: User)]
pub async fn get_wallet_handler() -> Result<WalletStateResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let wallet = DdbArcadeWallet::new(cli.clone());
    let user_id = user_inner_id(&user);
    let chip_balance = wallet.balance(&user_id).await.map_err(|e| {
        crate::error!("get_wallet_handler balance read failed: {e}");
        ArcadeError::StorageFailure
    })?;
    let settings = ArcadeSettings::get_or_default(cli).await.unwrap_or_default();

    Ok(WalletStateResponse {
        chip_balance,
        rp_to_chip_ratio_bps: settings.rp_to_chip_ratio_bps,
        redeem_enabled: settings.redeem_enabled,
    })
}

// ── POST /api/arcade/wallet/convert ──────────────────────────────────

#[post("/api/arcade/wallet/convert", user: User)]
pub async fn convert_rp_handler(req: ConvertRpRequest) -> Result<ConvertRpResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    if req.rp_amount <= 0 {
        return Err(ArcadeError::WalletAmountOutOfRange.into());
    }

    let wallet = DdbArcadeWallet::new(cli.clone());
    let user_id = user_inner_id(&user);
    let receipt = wallet
        .convert_rp_to_chip(&user_id, req.rp_amount)
        .await?;
    Ok(ConvertRpResponse {
        txn_id: receipt.txn_id,
        rp_debited: req.rp_amount,
        chips_credited: receipt.chips_credited,
        balance_after: receipt.balance_after,
    })
}

// ── POST /api/arcade/wallet/redeem ───────────────────────────────────
// v1: always returns WalletRedeemDisabled. Endpoint exists so clients
// can compile against the v2 surface and the feature flag flip
// happens server-side without an SDK release.

#[post("/api/arcade/wallet/redeem", user: User)]
pub async fn redeem_chip_handler(req: RedeemChipRequest) -> Result<ConvertRpResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let wallet = DdbArcadeWallet::new(cli.clone());
    let user_id = user_inner_id(&user);
    // Forward to the wallet impl so the same v2 flip works whether
    // the gate lives in the trait or the controller.
    let _ = wallet
        .redeem_chip_to_rp(&user_id, req.chip_amount)
        .await?;
    // Trait returns Err(WalletRedeemDisabled) today, so we never
    // reach here. Once enabled the response will mirror convert.
    unreachable!("redeem_chip_to_rp returned Ok while disabled");
}
