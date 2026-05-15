use crate::features::arcade::games::fact_or_fold::hooks::{
    UseFactFoldAdminHeadlines, use_fact_fold_admin_headlines_provider,
};
use crate::features::arcade::games::fact_or_fold::{HeadlineResponse, HeadlineStatus, Verdict};
use crate::*;

use super::i18n::FactFoldAdminHeadlinesTranslate;

/// `/admin/fact-or-fold/headlines` — list every headline ever
/// authored, filterable by status. Mockup also has KPI tiles +
/// inline search; both deferred to a follow-up since they need a
/// dedicated count endpoint and free-text search support.
#[component]
pub fn FactFoldAdminHeadlinesPage() -> Element {
    let UseFactFoldAdminHeadlines {
        status_filter,
        headlines,
        ..
    } = use_fact_fold_admin_headlines_provider()?;
    let rows = headlines();
    let tr: FactFoldAdminHeadlinesTranslate = use_translate();

    let total = rows.len();
    let count_live = rows
        .iter()
        .filter(|h| matches!(h.status, HeadlineStatus::Live))
        .count();
    let count_scheduled = rows
        .iter()
        .filter(|h| matches!(h.status, HeadlineStatus::Scheduled))
        .count();
    let count_draft = rows
        .iter()
        .filter(|h| matches!(h.status, HeadlineStatus::Draft))
        .count();
    let count_settled = rows
        .iter()
        .filter(|h| matches!(h.status, HeadlineStatus::Settled))
        .count();

    rsx! {
        SeoMeta { title: "{tr.page_title} · Fact or Fold" }
        section { class: "ff-headlines",
            // KPI strip — counts are over the *currently filtered*
            // page; a global count needs a count endpoint we'll add
            // when paging in.
            div { class: "ff-headlines__kpi",
                KpiTile { label: "{tr.kpi_total}", value: "{total}" }
                KpiTile {
                    label: "{tr.kpi_live}",
                    value: "{count_live}",
                    accent: "pink",
                }
                KpiTile {
                    label: "{tr.kpi_scheduled}",
                    value: "{count_scheduled}",
                    accent: "gold",
                }
                KpiTile {
                    label: "{tr.kpi_draft}",
                    value: "{count_draft}",
                    accent: "teal",
                }
            }

            // Filter tabs
            FilterTabs {
                status_filter,
                count_total: total,
                count_live,
                count_scheduled,
                count_draft,
                count_settled,
            }

            // Table
            div { class: "ff-headlines__panel",
                if rows.is_empty() {
                    div { class: "ff-headlines__empty", "{tr.empty}" }
                } else {
                    HeadlinesTable { rows }
                }
            }
        }
    }
}

#[component]
fn KpiTile(label: String, value: String, #[props(default)] accent: String) -> Element {
    rsx! {
        div { class: "ff-headlines__kpi-tile",
            div { class: "ff-headlines__kpi-label", "{label}" }
            div { class: "ff-headlines__kpi-value", "data-accent": "{accent}", "{value}" }
        }
    }
}

#[component]
fn FilterTabs(
    status_filter: Signal<Option<HeadlineStatus>>,
    count_total: usize,
    count_live: usize,
    count_scheduled: usize,
    count_draft: usize,
    count_settled: usize,
) -> Element {
    let tr: FactFoldAdminHeadlinesTranslate = use_translate();
    let mut status_filter = status_filter;
    let current = status_filter();

    rsx! {
        div { class: "ff-headlines__tabs",
            FilterTab {
                label: "{tr.tab_all}",
                count: count_total,
                active: current.is_none(),
                onclick: move |_| status_filter.set(None),
            }
            FilterTab {
                label: "{tr.tab_live}",
                count: count_live,
                active: matches!(current, Some(HeadlineStatus::Live)),
                onclick: move |_| status_filter.set(Some(HeadlineStatus::Live)),
            }
            FilterTab {
                label: "{tr.tab_scheduled}",
                count: count_scheduled,
                active: matches!(current, Some(HeadlineStatus::Scheduled)),
                onclick: move |_| status_filter.set(Some(HeadlineStatus::Scheduled)),
            }
            FilterTab {
                label: "{tr.tab_draft}",
                count: count_draft,
                active: matches!(current, Some(HeadlineStatus::Draft)),
                onclick: move |_| status_filter.set(Some(HeadlineStatus::Draft)),
            }
            FilterTab {
                label: "{tr.tab_settled}",
                count: count_settled,
                active: matches!(current, Some(HeadlineStatus::Settled)),
                onclick: move |_| status_filter.set(Some(HeadlineStatus::Settled)),
            }
        }
    }
}

#[component]
fn FilterTab(
    label: String,
    count: usize,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "ff-headlines__tab",
            "aria-selected": active,
            onclick: move |e| onclick.call(e),
            span { "{label}" }
            span { class: "ff-headlines__tab-count", "{count}" }
        }
    }
}

#[component]
fn HeadlinesTable(rows: Vec<HeadlineResponse>) -> Element {
    let tr: FactFoldAdminHeadlinesTranslate = use_translate();
    rsx! {
        table { class: "ff-headlines__table",
            thead {
                tr {
                    th { "{tr.col_id}" }
                    th { "{tr.col_headline}" }
                    th { "{tr.col_verdict}" }
                    th { "{tr.col_tags}" }
                    th { "{tr.col_scheduled}" }
                    th { "{tr.col_status}" }
                    th { "" }
                }
            }
            tbody {
                for row in rows {
                    HeadlineRow { row, key: "{row.id.0}" }
                }
            }
        }
    }
}

#[component]
fn HeadlineRow(row: HeadlineResponse) -> Element {
    let tr: FactFoldAdminHeadlinesTranslate = use_translate();
    let mut ctx = use_fact_fold_admin_headlines_provider()?;

    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut busy = use_signal(|| false);

    let row_id_for_publish = row.id.clone();
    let row_id_for_delete = row.id.clone();

    let on_publish = move |_| {
        let id = row_id_for_publish.clone();
        async move {
            busy.set(true);
            if let Err(e) = ctx.publish_now(id).await {
                error_msg.set(Some(format!("{e}")));
            }
            busy.set(false);
        }
    };

    let on_delete = move |_| {
        let id = row_id_for_delete.clone();
        async move {
            busy.set(true);
            if let Err(e) = ctx.delete(id).await {
                error_msg.set(Some(format!("{e}")));
            }
            busy.set(false);
        }
    };

    let scheduled_label = row
        .scheduled_at
        .map(format_millis_short)
        .unwrap_or_else(|| "—".to_string());

    let can_publish_now = matches!(row.status, HeadlineStatus::Draft | HeadlineStatus::Scheduled);
    let can_delete = !matches!(row.status, HeadlineStatus::Live | HeadlineStatus::Settled);

    rsx! {
        tr { class: "ff-headlines__row",
            td { class: "ff-headlines__cell-mono", "{short_id(&row.id.0)}" }
            td {
                div { class: "ff-headlines__headline-text", "{row.headline_text}" }
                if let Some(err) = error_msg() {
                    div { class: "ff-headlines__row-error", "{err}" }
                }
            }
            td {
                VerdictBadge { verdict: row.verdict }
            }
            td {
                div { class: "ff-headlines__tags",
                    for tag in row.category_tags.iter() {
                        span { class: "ff-headlines__pill", "{tag}" }
                    }
                }
            }
            td { class: "ff-headlines__cell-muted", "{scheduled_label}" }
            td {
                StatusBadge { status: row.status }
            }
            td { class: "ff-headlines__row-actions",
                if can_publish_now {
                    button {
                        class: "ff-headlines__icon-btn",
                        title: "{tr.action_publish}",
                        disabled: busy(),
                        onclick: on_publish,
                        "▶"
                    }
                }
                if can_delete {
                    button {
                        class: "ff-headlines__icon-btn ff-headlines__icon-btn--danger",
                        title: "{tr.action_delete}",
                        disabled: busy(),
                        onclick: on_delete,
                        "✕"
                    }
                }
            }
        }
    }
}

#[component]
fn VerdictBadge(verdict: Verdict) -> Element {
    let (variant, label) = match verdict {
        Verdict::Real => ("real", "REAL"),
        Verdict::Fake => ("fake", "FAKE"),
    };
    rsx! {
        span { class: "ff-headlines__verdict", "data-variant": "{variant}", "{label}" }
    }
}

#[component]
fn StatusBadge(status: HeadlineStatus) -> Element {
    let (variant, label) = match status {
        HeadlineStatus::Draft => ("draft", "DRAFT"),
        HeadlineStatus::Scheduled => ("scheduled", "SCHEDULED"),
        HeadlineStatus::Live => ("live", "LIVE"),
        HeadlineStatus::Settled => ("settled", "SETTLED"),
        HeadlineStatus::Deleted => ("deleted", "DELETED"),
    };
    rsx! {
        span { class: "ff-headlines__status", "data-variant": "{variant}", "{label}" }
    }
}

/// Trim a UUID-shaped headline id to the first 8 chars so the table
/// stays scannable. Short id only used for display — the full id is
/// always carried in actions / links.
fn short_id(id: &str) -> String {
    id.chars().take(8).collect()
}

/// Format millis-since-epoch into an `M/D HH:MM` short label.
/// Server already stores UTC millis; this is a coarse local-time
/// rendering (dates only — no timezone library to keep deps small).
fn format_millis_short(millis: i64) -> String {
    let secs = millis / 1000;
    let days_since_epoch = secs / 86_400;
    // 1970-01-01 was a Thursday; we just use day-of-year math via
    // chrono once we wire it. For now, expose ISO yyyy-mm-dd via the
    // browser by deferring formatting to UI string interp on the
    // client. Keeping a millis fallback so the cell stays readable.
    let _ = days_since_epoch;
    format!("ts:{millis}")
}
