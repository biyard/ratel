//! `UseFactFoldAdminSchedule` — Schedule view's data:
//!  - List of `Scheduled` headlines sorted by `scheduled_at` asc
//!  - Queue alarm payload (drives the FR-45 banner)
//!
//! Separate from `UseFactFoldAdminHeadlines` because the schedule
//! view always pins the filter to `Scheduled` and re-sorts by
//! publication time, not creation time.

use crate::features::arcade::games::fact_or_fold::{
    HeadlineResponse, HeadlineStatus, QueueAlarmResponse, get_queue_alarm_handler,
    list_headlines_handler,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldAdminSchedule {
    pub scheduled: Loader<Vec<HeadlineResponse>>,
    pub alarm: Loader<QueueAlarmResponse>,
}

pub fn use_fact_fold_admin_schedule_provider()
-> std::result::Result<UseFactFoldAdminSchedule, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldAdminSchedule>() {
        return Ok(ctx);
    }

    let scheduled = use_loader(move || async move {
        let resp = list_headlines_handler(None, Some(HeadlineStatus::Scheduled)).await?;
        let mut items = resp.items;
        items.sort_by(|a, b| a.scheduled_at.unwrap_or(0).cmp(&b.scheduled_at.unwrap_or(0)));
        Ok::<Vec<HeadlineResponse>, crate::common::Error>(items)
    })?;

    let alarm = use_loader(move || async move { get_queue_alarm_handler().await })?;

    Ok(use_context_provider(|| UseFactFoldAdminSchedule {
        scheduled,
        alarm,
    }))
}
