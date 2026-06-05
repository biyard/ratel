//! Admin settings endpoints for *Fact or Fold*.
//!
//! Surface:
//!  - GET /api/fact-or-fold/admin/settings   read singleton (falls back to default)
//!  - PUT /api/fact-or-fold/admin/settings   upsert with partial fields
//!
//! Gated by `AdminUser` — roadmap §FR-39 + Tunable parameters table.

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::AdminUser;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::FactFoldSettings;

// ── Helpers ───────────────────────────────────────────────────────

#[cfg(feature = "server")]
fn validate_settings(s: &FactOrFoldSettingsResponse) -> Result<()> {
    // round_capacity == 1 is allowed for solo / dev modes — the
    // insider role just becomes the lone player. Admin UI surfaces
    // a warning when 1 is chosen so the operator knows it bypasses
    // the regular mafia-style dynamic.
    let ranges_ok = s.round_capacity >= 1
        && s.round_capacity <= 8
        && s.stage_news_reveal_sec > 0
        && s.stage_bet_sec > 0
        && s.stage_rationale_sec > 0
        && s.stage_reveal_sec > 0
        && s.stage_debate_sec > 0
        && s.min_bet_rp > 0
        && s.max_bet_rp >= s.min_bet_rp
        && s.correct_side_multiplier_bps >= 10_000
        && s.insider_correct_bonus_bps >= 0
        && s.influence_bonus_bps >= 0
        && s.new_user_signup_rp >= 0
        && s.reconnect_grace_sec > 0
        && s.queue_low_alert_days > 0;
    if !ranges_ok {
        return Err(FactOrFoldError::SettingsOutOfRange.into());
    }
    Ok(())
}

#[cfg(feature = "server")]
fn apply_patch(
    current: FactOrFoldSettingsResponse,
    req: UpdateFactOrFoldSettingsRequest,
) -> FactOrFoldSettingsResponse {
    FactOrFoldSettingsResponse {
        round_capacity: req.round_capacity.unwrap_or(current.round_capacity),
        stage_news_reveal_sec: req
            .stage_news_reveal_sec
            .unwrap_or(current.stage_news_reveal_sec),
        stage_bet_sec: req.stage_bet_sec.unwrap_or(current.stage_bet_sec),
        stage_rationale_sec: req
            .stage_rationale_sec
            .unwrap_or(current.stage_rationale_sec),
        stage_reveal_sec: req.stage_reveal_sec.unwrap_or(current.stage_reveal_sec),
        stage_debate_sec: req.stage_debate_sec.unwrap_or(current.stage_debate_sec),
        min_bet_rp: req.min_bet_rp.unwrap_or(current.min_bet_rp),
        max_bet_rp: req.max_bet_rp.unwrap_or(current.max_bet_rp),
        correct_side_multiplier_bps: req
            .correct_side_multiplier_bps
            .unwrap_or(current.correct_side_multiplier_bps),
        insider_correct_bonus_bps: req
            .insider_correct_bonus_bps
            .unwrap_or(current.insider_correct_bonus_bps),
        influence_bonus_bps: req
            .influence_bonus_bps
            .unwrap_or(current.influence_bonus_bps),
        new_user_signup_rp: req.new_user_signup_rp.unwrap_or(current.new_user_signup_rp),
        reconnect_grace_sec: req
            .reconnect_grace_sec
            .unwrap_or(current.reconnect_grace_sec),
        queue_low_alert_days: req
            .queue_low_alert_days
            .unwrap_or(current.queue_low_alert_days),
    }
}

// ── GET /api/fact-or-fold/admin/settings ─────────────────────────

#[get("/api/fact-or-fold/admin/settings", _user: AdminUser)]
pub async fn get_settings_handler() -> Result<FactOrFoldSettingsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    FactFoldSettings::get_or_default(cli).await.map_err(|e| {
        crate::error!("get_settings_handler failed: {e}");
        FactOrFoldError::StorageFailure.into()
    })
}

// ── PUT /api/fact-or-fold/admin/settings ─────────────────────────

#[put("/api/fact-or-fold/admin/settings", _user: AdminUser)]
pub async fn update_settings_handler(
    req: UpdateFactOrFoldSettingsRequest,
) -> Result<FactOrFoldSettingsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let current = FactFoldSettings::get_or_default(cli).await.map_err(|e| {
        crate::error!("update_settings_handler read failed: {e}");
        FactOrFoldError::StorageFailure
    })?;
    let next = apply_patch(current, req);
    validate_settings(&next)?;

    let row = FactFoldSettings::default().merge_response(next.clone());
    row.upsert(cli).await.map_err(|e| {
        crate::error!("update_settings_handler put failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(next)
}
