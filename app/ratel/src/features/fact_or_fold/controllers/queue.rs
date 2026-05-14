//! Queue health endpoint for *Fact or Fold* admin.
//!
//! Surface:
//!  - GET /api/fact-or-fold/admin/queue/alarm
//!
//! Drives the FR-45 alert: when the latest *Scheduled* headline's
//! `scheduled_at` falls within `queue_low_alert_days` of "now" (or no
//! scheduled headline exists at all), the admin UI surfaces a warning
//! prompting the operator to publish more.

use crate::common::*;
use crate::features::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::AdminUser;
#[cfg(feature = "server")]
use crate::features::fact_or_fold::models::{FactFoldHeadline, FactFoldSettings};

#[cfg(feature = "server")]
const HEADLINE_SK_PREFIX: &str = "FACT_FOLD_HEADLINE";
#[cfg(feature = "server")]
const MS_PER_DAY: f64 = 86_400_000.0;

#[get("/api/fact-or-fold/admin/queue/alarm", _user: AdminUser)]
pub async fn get_queue_alarm_handler() -> Result<QueueAlarmResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let settings = FactFoldSettings::get_or_default(cli).await.map_err(|e| {
        crate::error!("get_queue_alarm_handler settings read failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    // We anticipate ≤ a few hundred headlines lifetime. One sk-prefix
    // query is fine without pagination for an alarm check.
    let opts = FactFoldHeadline::opt()
        .sk(HEADLINE_SK_PREFIX.to_string())
        .limit(500);
    let (rows, _) = FactFoldHeadline::query(cli, FactFoldHeadline::anchor_pk(), opts)
        .await
        .map_err(|e| {
            crate::error!("get_queue_alarm_handler query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let max_scheduled_at = rows
        .iter()
        .filter(|h| matches!(h.status, HeadlineStatus::Scheduled))
        .filter_map(|h| h.scheduled_at)
        .filter(|ts| *ts >= now)
        .max();
    let scheduled_future_count = rows
        .iter()
        .filter(|h| matches!(h.status, HeadlineStatus::Scheduled))
        .filter(|h| h.scheduled_at.map(|ts| ts >= now).unwrap_or(false))
        .count();

    let queue_days_remaining = match max_scheduled_at {
        Some(ts) => ((ts - now) as f64 / MS_PER_DAY).max(0.0),
        None => 0.0,
    };
    let alert = queue_days_remaining <= settings.queue_low_alert_days as f64;

    Ok(QueueAlarmResponse {
        queue_days_remaining,
        alert_threshold_days: settings.queue_low_alert_days,
        alert,
        scheduled_future_count: scheduled_future_count as i32,
    })
}
