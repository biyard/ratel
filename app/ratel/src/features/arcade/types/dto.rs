//! Request / response DTOs for arcade-level endpoints (wallet,
//! settings). Game-specific DTOs live under `games::<name>::types`.

use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

// ── Wallet ──────────────────────────────────────────────────────────

/// Snapshot returned by `GET /api/arcade/wallet`.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WalletStateResponse {
    /// Current spendable chip balance.
    pub chip_balance: i64,
    /// Operator's RP→chip conversion ratio in basis points (10_000 = 1×).
    /// Surfaced so the client can show "1 RP = N chip" without a
    /// settings round-trip.
    pub rp_to_chip_ratio_bps: i32,
    /// Whether chip→RP redeem is currently enabled (v1 = false).
    pub redeem_enabled: bool,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConvertRpRequest {
    /// RP to debit. Must be >= `min_convert_rp` and <= caller's RP
    /// balance.
    pub rp_amount: i64,
}

/// Result of a successful RP → chip conversion.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConvertRpResponse {
    pub txn_id: String,
    pub rp_debited: i64,
    pub chips_credited: i64,
    pub balance_after: i64,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RedeemChipRequest {
    pub chip_amount: i64,
}

// ── Settings ────────────────────────────────────────────────────────

/// Operator-tunable response surface mirrored from the
/// server-side `ArcadeSettings` entity. Defaults match v1's "1 RP =
/// 1 chip" decision.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArcadeSettingsResponse {
    /// Chips received per 1 RP converted, in basis points (10_000 =
    /// 1.0×). v1 default is 10_000 so 1 RP → 1 chip. Operator can
    /// tune to make chips cheaper / more expensive in future seasons.
    pub rp_to_chip_ratio_bps: i32,

    /// Default chip amount auto-bought-in at lobby join when the
    /// caller doesn't pick one. Games can override per-round.
    pub default_buy_in_chips: i64,

    /// Minimum convert amount in RP. Prevents dust conversions.
    pub min_convert_rp: i64,

    /// Whether chip → RP redemption is enabled. v1 = false.
    pub redeem_enabled: bool,
}

impl Default for ArcadeSettingsResponse {
    fn default() -> Self {
        Self {
            rp_to_chip_ratio_bps: 10_000, // 1:1
            default_buy_in_chips: 100,
            min_convert_rp: 100,
            redeem_enabled: false,
        }
    }
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateArcadeSettingsRequest {
    pub rp_to_chip_ratio_bps: Option<i32>,
    pub default_buy_in_chips: Option<i64>,
    pub min_convert_rp: Option<i64>,
    pub redeem_enabled: Option<bool>,
}
