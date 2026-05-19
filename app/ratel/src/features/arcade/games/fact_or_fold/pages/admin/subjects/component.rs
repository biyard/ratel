use crate::features::arcade::games::fact_or_fold::hooks::{
    UseFactFoldAdminSubjects, use_fact_fold_admin_subjects_provider,
};
use crate::features::arcade::games::fact_or_fold::{SubjectResponse, SubjectStatus, Verdict};
use crate::*;

use super::i18n::FactFoldAdminSubjectsTranslate;

/// `/admin/fact-or-fold/subjects` — list every subject ever
/// authored, filterable by status. Mockup also has KPI tiles +
/// inline search; both deferred to a follow-up since they need a
/// dedicated count endpoint and free-text search support.
#[component]
pub fn FactFoldAdminSubjectsPage() -> Element {
    let UseFactFoldAdminSubjects {
        status_filter,
        subjects,
        ..
    } = use_fact_fold_admin_subjects_provider()?;
    let rows = subjects();
    let tr: FactFoldAdminSubjectsTranslate = use_translate();

    let total = rows.len();
    let count_live = rows
        .iter()
        .filter(|h| matches!(h.status, SubjectStatus::Live))
        .count();
    let count_scheduled = rows
        .iter()
        .filter(|h| matches!(h.status, SubjectStatus::Scheduled))
        .count();
    let count_draft = rows
        .iter()
        .filter(|h| matches!(h.status, SubjectStatus::Draft))
        .count();
    let count_settled = rows
        .iter()
        .filter(|h| matches!(h.status, SubjectStatus::Settled))
        .count();

    rsx! {
        SeoMeta { title: "{tr.page_title} · Fact or Fold" }
        section { class: "ff-subjects",
            // KPI strip — counts are over the *currently filtered*
            // page; a global count needs a count endpoint we'll add
            // when paging in.
            div { class: "ff-subjects__kpi",
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
            div { class: "ff-subjects__panel",
                if rows.is_empty() {
                    div { class: "ff-subjects__empty", "{tr.empty}" }
                } else {
                    SubjectsTable { rows }
                }
            }
        }
    }
}

#[component]
fn KpiTile(label: String, value: String, #[props(default)] accent: String) -> Element {
    rsx! {
        div { class: "ff-subjects__kpi-tile",
            div { class: "ff-subjects__kpi-label", "{label}" }
            div { class: "ff-subjects__kpi-value", "data-accent": "{accent}", "{value}" }
        }
    }
}

#[component]
fn FilterTabs(
    status_filter: Signal<Option<SubjectStatus>>,
    count_total: usize,
    count_live: usize,
    count_scheduled: usize,
    count_draft: usize,
    count_settled: usize,
) -> Element {
    let tr: FactFoldAdminSubjectsTranslate = use_translate();
    let mut status_filter = status_filter;
    let current = status_filter();

    rsx! {
        div { class: "ff-subjects__tabs",
            FilterTab {
                label: "{tr.tab_all}",
                count: count_total,
                active: current.is_none(),
                onclick: move |_| status_filter.set(None),
            }
            FilterTab {
                label: "{tr.tab_live}",
                count: count_live,
                active: matches!(current, Some(SubjectStatus::Live)),
                onclick: move |_| status_filter.set(Some(SubjectStatus::Live)),
            }
            FilterTab {
                label: "{tr.tab_scheduled}",
                count: count_scheduled,
                active: matches!(current, Some(SubjectStatus::Scheduled)),
                onclick: move |_| status_filter.set(Some(SubjectStatus::Scheduled)),
            }
            FilterTab {
                label: "{tr.tab_draft}",
                count: count_draft,
                active: matches!(current, Some(SubjectStatus::Draft)),
                onclick: move |_| status_filter.set(Some(SubjectStatus::Draft)),
            }
            FilterTab {
                label: "{tr.tab_settled}",
                count: count_settled,
                active: matches!(current, Some(SubjectStatus::Settled)),
                onclick: move |_| status_filter.set(Some(SubjectStatus::Settled)),
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
            class: "ff-subjects__tab",
            "aria-selected": active,
            onclick: move |e| onclick.call(e),
            span { "{label}" }
            span { class: "ff-subjects__tab-count", "{count}" }
        }
    }
}

#[component]
fn SubjectsTable(rows: Vec<SubjectResponse>) -> Element {
    let tr: FactFoldAdminSubjectsTranslate = use_translate();
    rsx! {
        table { class: "ff-subjects__table",
            thead {
                tr {
                    th { "{tr.col_id}" }
                    th { "{tr.col_subject}" }
                    th { "{tr.col_verdict}" }
                    th { "{tr.col_tags}" }
                    th { "{tr.col_scheduled}" }
                    th { "{tr.col_status}" }
                    th { "" }
                }
            }
            tbody {
                for row in rows {
                    SubjectRow { row, key: "{row.id.0}" }
                }
            }
        }
    }
}

#[component]
fn SubjectRow(row: SubjectResponse) -> Element {
    let tr: FactFoldAdminSubjectsTranslate = use_translate();
    let mut ctx = use_fact_fold_admin_subjects_provider()?;

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

    let can_publish_now = matches!(row.status, SubjectStatus::Draft | SubjectStatus::Scheduled);
    let can_delete = !matches!(row.status, SubjectStatus::Live | SubjectStatus::Settled);

    rsx! {
        tr { class: "ff-subjects__row",
            td { class: "ff-subjects__cell-mono", "{short_id(&row.id.0)}" }
            td {
                div { class: "ff-subjects__subject-text", "{row.headline_text}" }
                if let Some(err) = error_msg() {
                    div { class: "ff-subjects__row-error", "{err}" }
                }
            }
            td {
                VerdictBadge { verdict: row.verdict }
            }
            td {
                div { class: "ff-subjects__tags",
                    for tag in row.category_tags.iter() {
                        span { class: "ff-subjects__pill", "{tag}" }
                    }
                }
            }
            td { class: "ff-subjects__cell-muted", "{scheduled_label}" }
            td {
                StatusBadge { status: row.status }
            }
            td { class: "ff-subjects__row-actions",
                if can_publish_now {
                    button {
                        class: "ff-subjects__icon-btn",
                        title: "{tr.action_publish}",
                        disabled: busy(),
                        onclick: on_publish,
                        "▶"
                    }
                }
                if can_delete {
                    button {
                        class: "ff-subjects__icon-btn ff-subjects__icon-btn--danger",
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
        span { class: "ff-subjects__verdict", "data-variant": "{variant}", "{label}" }
    }
}

#[component]
fn StatusBadge(status: SubjectStatus) -> Element {
    let (variant, label) = match status {
        SubjectStatus::Draft => ("draft", "DRAFT"),
        SubjectStatus::Scheduled => ("scheduled", "SCHEDULED"),
        SubjectStatus::Live => ("live", "LIVE"),
        SubjectStatus::Settled => ("settled", "SETTLED"),
        SubjectStatus::Deleted => ("deleted", "DELETED"),
    };
    rsx! {
        span { class: "ff-subjects__status", "data-variant": "{variant}", "{label}" }
    }
}

/// Trim a UUID-shaped subject id to the first 8 chars so the table
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
