use crate::features::arcade::games::fact_or_fold::hooks::{
    UseFactFoldAdminSchedule, use_fact_fold_admin_schedule_provider,
};
use crate::route::Route;
use crate::*;

use super::i18n::FactFoldAdminScheduleTranslate;

/// `/admin/fact-or-fold/schedule` — chronologically-sorted list of
/// scheduled headlines + the FR-45 queue alarm banner. Calendar /
/// drag-to-reschedule view is deferred — needs a date library.
#[component]
pub fn FactFoldAdminSchedulePage() -> Element {
    let UseFactFoldAdminSchedule { scheduled, alarm } = use_fact_fold_admin_schedule_provider()?;
    let tr: FactFoldAdminScheduleTranslate = use_translate();
    let rows = scheduled();
    let alarm = alarm();

    rsx! {
        SeoMeta { title: "{tr.page_title} · Fact or Fold" }
        section { class: "ff-schedule",
            // Queue alarm banner — only renders when alarm fires.
            if alarm.alert {
                AlarmBanner {
                    days_remaining: alarm.queue_days_remaining,
                    alert_threshold_days: alarm.alert_threshold_days,
                    scheduled_count: alarm.scheduled_future_count,
                }
            } else {
                HealthyBanner {
                    days_remaining: alarm.queue_days_remaining,
                    scheduled_count: alarm.scheduled_future_count,
                }
            }

            // Upcoming list
            div { class: "ff-schedule__panel",
                header { class: "ff-schedule__panel-head",
                    span { class: "ff-schedule__panel-title", "{tr.upcoming_title}" }
                    span { class: "ff-schedule__panel-sub", "{rows.len()} {tr.upcoming_count_suffix}" }
                }
                if rows.is_empty() {
                    div { class: "ff-schedule__empty", "{tr.empty}" }
                } else {
                    ul { class: "ff-schedule__list",
                        for row in rows.iter() {
                            ScheduleRow { row: row.clone(), key: "{row.id.0}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AlarmBanner(
    days_remaining: f64,
    alert_threshold_days: i32,
    scheduled_count: i32,
) -> Element {
    let tr: FactFoldAdminScheduleTranslate = use_translate();
    let nav = use_navigator();
    let r_new = Route::FactFoldAdminNewHeadlinePage {};
    let days_str = format!("{:.1}", days_remaining);
    rsx! {
        div { class: "ff-schedule__banner ff-schedule__banner--alert",
            div { class: "ff-schedule__banner-icon", "⚠" }
            div { class: "ff-schedule__banner-text",
                strong { "{tr.alarm_title}" }
                p {
                    "{tr.alarm_body_prefix} "
                    strong { "{days_str} {tr.alarm_body_days_unit}" }
                    " ({scheduled_count} {tr.alarm_body_count_suffix}). "
                    "{tr.alarm_threshold_prefix} {alert_threshold_days}{tr.alarm_threshold_suffix}"
                }
            }
            button {
                class: "ff-schedule__banner-cta",
                onclick: {
                    let r = r_new.clone();
                    move |_| {
                        nav.push(r.clone());
                    }
                },
                "+ {tr.alarm_cta}"
            }
        }
    }
}

#[component]
fn HealthyBanner(days_remaining: f64, scheduled_count: i32) -> Element {
    let tr: FactFoldAdminScheduleTranslate = use_translate();
    let days_str = format!("{:.1}", days_remaining);
    rsx! {
        div { class: "ff-schedule__banner ff-schedule__banner--ok",
            div { class: "ff-schedule__banner-icon", "✓" }
            div { class: "ff-schedule__banner-text",
                strong { "{tr.healthy_title}" }
                p {
                    "{scheduled_count} {tr.healthy_count_suffix} · {days_str} {tr.healthy_days_suffix}"
                }
            }
        }
    }
}

#[component]
fn ScheduleRow(row: crate::features::arcade::games::fact_or_fold::HeadlineResponse) -> Element {
    let scheduled_label = row
        .scheduled_at
        .map(|ts| format!("ts:{ts}"))
        .unwrap_or_else(|| "—".to_string());
    rsx! {
        li { class: "ff-schedule__row",
            div { class: "ff-schedule__when", "{scheduled_label}" }
            div { class: "ff-schedule__what",
                div { class: "ff-schedule__row-title", "{row.headline_text}" }
                div { class: "ff-schedule__row-meta",
                    span { class: "ff-schedule__row-source", "{row.source_label}" }
                    if !row.category_tags.is_empty() {
                        span { class: "ff-schedule__row-sep", "·" }
                        for tag in row.category_tags.iter() {
                            span { class: "ff-schedule__row-tag", "{tag}" }
                        }
                    }
                }
            }
        }
    }
}
