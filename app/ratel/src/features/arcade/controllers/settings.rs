//! Arcade-wide settings endpoints (operator-only).
//!
//! Surface:
//!   GET   /api/arcade/admin/settings
//!   PUT   /api/arcade/admin/settings

use crate::common::*;
use crate::features::arcade::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::AdminUser;
#[cfg(feature = "server")]
use crate::features::arcade::ArcadeError;
#[cfg(feature = "server")]
use crate::features::arcade::models::ArcadeSettings;

// ── Helpers ─────────────────────────────────────────────────────────

#[cfg(feature = "server")]
fn validate_settings(s: &ArcadeSettingsResponse) -> Result<()> {
    let ok = s.rp_to_chip_ratio_bps > 0
        && s.default_buy_in_chips > 0
        && s.min_convert_rp > 0;
    if !ok {
        return Err(ArcadeError::WalletAmountOutOfRange.into());
    }
    Ok(())
}

#[cfg(feature = "server")]
fn apply_patch(
    current: ArcadeSettingsResponse,
    req: UpdateArcadeSettingsRequest,
) -> ArcadeSettingsResponse {
    ArcadeSettingsResponse {
        rp_to_chip_ratio_bps: req
            .rp_to_chip_ratio_bps
            .unwrap_or(current.rp_to_chip_ratio_bps),
        default_buy_in_chips: req
            .default_buy_in_chips
            .unwrap_or(current.default_buy_in_chips),
        min_convert_rp: req.min_convert_rp.unwrap_or(current.min_convert_rp),
        redeem_enabled: req.redeem_enabled.unwrap_or(current.redeem_enabled),
    }
}

#[cfg(feature = "server")]
fn merge_response(mut row: ArcadeSettings, r: ArcadeSettingsResponse) -> ArcadeSettings {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    row.rp_to_chip_ratio_bps = r.rp_to_chip_ratio_bps;
    row.default_buy_in_chips = r.default_buy_in_chips;
    row.min_convert_rp = r.min_convert_rp;
    row.redeem_enabled = r.redeem_enabled;
    if row.created_at == 0 {
        row.created_at = now;
    }
    row.updated_at = now;
    let (pk, sk) = ArcadeSettings::keys();
    row.pk = pk;
    row.sk = sk;
    row
}

// ── GET /api/arcade/admin/settings ───────────────────────────────────

#[get("/api/arcade/admin/settings", _user: AdminUser)]
pub async fn get_arcade_settings_handler() -> Result<ArcadeSettingsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    ArcadeSettings::get_or_default(cli).await.map_err(|e| {
        crate::error!("get_arcade_settings_handler failed: {e}");
        ArcadeError::StorageFailure.into()
    })
}

// ── PUT /api/arcade/admin/settings ───────────────────────────────────

#[put("/api/arcade/admin/settings", _user: AdminUser)]
pub async fn update_arcade_settings_handler(
    req: UpdateArcadeSettingsRequest,
) -> Result<ArcadeSettingsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let current = ArcadeSettings::get_or_default(cli).await.map_err(|e| {
        crate::error!("update_arcade_settings read failed: {e}");
        ArcadeError::StorageFailure
    })?;
    let next = apply_patch(current, req);
    validate_settings(&next)?;

    let row = merge_response(ArcadeSettings::default(), next.clone());
    row.upsert(cli).await.map_err(|e| {
        crate::error!("update_arcade_settings put failed: {e}");
        ArcadeError::StorageFailure
    })?;

    Ok(next)
}
