//! `UseFactFoldAdminSchedule` — Schedule view's data:
//!  - List of `Scheduled` subjects sorted by `scheduled_at` asc
//!  - Queue alarm payload (drives the FR-45 banner)
//!
//! Separate from `UseFactFoldAdminSubjects` because the schedule
//! view always pins the filter to `Scheduled` and re-sorts by
//! publication time, not creation time.

use crate::features::arcade::games::fact_or_fold::{
    SubjectResponse, SubjectStatus, QueueAlarmResponse, get_queue_alarm_handler,
    list_subjects_handler,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldAdminSchedule {
    pub scheduled: Loader<Vec<SubjectResponse>>,
    pub alarm: Loader<QueueAlarmResponse>,
}

pub fn use_fact_fold_admin_schedule_provider()
-> std::result::Result<UseFactFoldAdminSchedule, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldAdminSchedule>() {
        return Ok(ctx);
    }

    let scheduled = use_loader(move || async move {
        let resp = list_subjects_handler(None, Some(SubjectStatus::Scheduled)).await?;
        let mut items = resp.items;
        items.sort_by(|a, b| a.scheduled_at.unwrap_or(0).cmp(&b.scheduled_at.unwrap_or(0)));
        Ok::<Vec<SubjectResponse>, crate::common::Error>(items)
    })?;

    let alarm = use_loader(move || async move { get_queue_alarm_handler().await })?;

    Ok(use_context_provider(|| UseFactFoldAdminSchedule {
        scheduled,
        alarm,
    }))
}
