//! "Pending applications / 신청 대기" tab — consumes `UseSubTeamQueue`.
//!
//! Mirrors `subteam-management-page.html`'s `.queue-row` block: avatar
//! + name + member-count/agreement/time meta + action row (review →
//! arena modal showing submitted answers, then approve / return /
//! reject with inline comment box).

use crate::features::sub_team::{
    use_sub_team_queue, SubTeamApplicationResponse, SubTeamFormFieldSnapshotDto, SubTeamTranslate,
    UseSubTeamQueue,
};
use crate::*;

#[component]
pub fn QueueTab() -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamQueue {
        mut queue,
        mut handle_approve,
        mut handle_reject,
        mut handle_return,
        ..
    } = use_sub_team_queue()?;

    let items = queue.items();
    let item_count = items.len();

    // Modal state — Some(application) means the review modal is open.
    let mut review_open: Signal<Option<SubTeamApplicationResponse>> = use_signal(|| None);

    rsx! {
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.tab_queue}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{item_count}" }
            }

            if item_count == 0 && !queue.is_loading() {
                div { class: "inline-note", "{tr.empty_list}" }
            } else {
                div { class: "u-col u-gap-10", id: "queue-list",
                    for app in items.iter() {
                        QueueRow {
                            key: "{app.id}",
                            app: app.clone(),
                            on_review: move |a: SubTeamApplicationResponse| {
                                review_open.set(Some(a));
                            },
                            on_approve: move |(id, msg)| handle_approve.call(id, msg),
                            on_reject: move |(id, reason)| handle_reject.call(id, reason),
                            on_return: move |(id, comment)| handle_return.call(id, comment),
                        }
                    }
                }
                {queue.more_element()}
            }
        }

        // Arena-style review modal — `data-open` toggles visibility.
        if let Some(app) = review_open() {
            ReviewApplicationModal { app: app.clone(), on_close: move |_| review_open.set(None) }
        }
    }
}

#[component]
fn QueueRow(
    app: SubTeamApplicationResponse,
    on_review: EventHandler<SubTeamApplicationResponse>,
    on_approve: EventHandler<(String, String)>,
    on_reject: EventHandler<(String, String)>,
    on_return: EventHandler<(String, String)>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let app_id = app.id.clone();
    let app_id_for_approve = app_id.clone();
    let app_id_for_return = app_id.clone();
    let app_id_for_reject = app_id.clone();
    let app_for_review = app.clone();

    let mut mode: Signal<QueueRowMode> = use_signal(|| QueueRowMode::Idle);
    let mut text: Signal<String> = use_signal(String::new);

    // Display fields — fall back gracefully when the join couldn't fill
    // the applicant team metadata server-side.
    let display_name = if app.applicant_team_display_name.is_empty() {
        format!("Application {}", short_id(&app.id))
    } else {
        app.applicant_team_display_name.clone()
    };
    let initials = team_initials(&display_name);
    let avatar_variant = avatar_variant_for(&app.id);
    let members_line = format!(
        "{} {}",
        app.applicant_member_count.max(0),
        tr.queue_row_members_suffix
    );
    let when_line = format_when(app.submitted_at.unwrap_or(app.created_at));
    let snapshot_line = format!(
        "{} · {} {}",
        tr.queue_row_form_snapshot,
        app.form_snapshot.len(),
        tr.review_modal_fields_suffix
    );

    rsx! {
        div { class: "queue-row", "data-testid": "sub-team-queue-row",
            div { class: "avatar {avatar_variant}", "{initials}" }
            div { class: "queue-row__body",
                span { class: "queue-row__name", "{display_name}" }
                div { class: "queue-row__meta",
                    span { "{members_line}" }
                    span { "{when_line}" }
                    span { "{snapshot_line}" }
                }
            }
            div { class: "queue-row__actions",
                button {
                    class: "queue-row__action queue-row__action--review",
                    "data-testid": "sub-team-queue-review-btn",
                    r#type: "button",
                    onclick: move |_| on_review.call(app_for_review.clone()),
                    "{tr.queue_row_review_btn}"
                }
                button {
                    class: "queue-row__action queue-row__action--approve",
                    "data-testid": "sub-team-queue-approve-btn",
                    "aria-pressed": "{mode() == QueueRowMode::Approve}",
                    r#type: "button",
                    onclick: move |_| {
                        mode.set(QueueRowMode::Approve);
                        text.set(String::new());
                    },
                    "{tr.approve}"
                }
                button {
                    class: "queue-row__action queue-row__action--return",
                    "data-testid": "sub-team-queue-return-btn",
                    "aria-pressed": "{mode() == QueueRowMode::Return}",
                    r#type: "button",
                    onclick: move |_| {
                        mode.set(QueueRowMode::Return);
                        text.set(String::new());
                    },
                    "{tr.r#return}"
                }
                button {
                    class: "queue-row__action queue-row__action--reject",
                    "data-testid": "sub-team-queue-reject-btn",
                    "aria-pressed": "{mode() == QueueRowMode::Reject}",
                    r#type: "button",
                    onclick: move |_| {
                        mode.set(QueueRowMode::Reject);
                        text.set(String::new());
                    },
                    "{tr.reject}"
                }
            }
            if mode() != QueueRowMode::Idle {
                {
                    let placeholder = match mode() {
                        QueueRowMode::Approve => tr.queue_row_approve_placeholder.to_string(),
                        QueueRowMode::Return => tr.queue_row_return_placeholder.to_string(),
                        QueueRowMode::Reject => tr.queue_row_reject_placeholder.to_string(),
                        QueueRowMode::Idle => String::new(),
                    };
                    rsx! {
                        div { class: "queue-row__decision",
                            textarea {
                                class: "field__textarea",
                                "data-testid": "sub-team-queue-decision-text",
                                placeholder: "{placeholder}",
                                value: "{text()}",
                                oninput: move |e| text.set(e.value()),
                            }
                            div { class: "u-flex u-gap-10",
                                button {
                                    class: "btn btn--ghost btn--small",
                                    r#type: "button",
                                    onclick: move |_| mode.set(QueueRowMode::Idle),
                                    "{tr.cancel}"
                                }
                                button {
                                    class: "btn btn--primary btn--small",
                                    "data-testid": "sub-team-queue-decision-confirm",
                                    r#type: "button",
                                    onclick: move |_| {
                                        let content = text().clone();
                                        match mode() {
                                            QueueRowMode::Approve => on_approve.call((app_id_for_approve.clone(), content)),
                                            QueueRowMode::Return => on_return.call((app_id_for_return.clone(), content)),
                                            QueueRowMode::Reject => on_reject.call((app_id_for_reject.clone(), content)),
                                            QueueRowMode::Idle => {}
                                        }
                                        mode.set(QueueRowMode::Idle);
                                        text.set(String::new());
                                    },
                                    "{tr.queue_row_confirm}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ReviewApplicationModal(
    app: SubTeamApplicationResponse,
    on_close: EventHandler<()>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let display_name = if app.applicant_team_display_name.is_empty() {
        format!("Application {}", short_id(&app.id))
    } else {
        app.applicant_team_display_name.clone()
    };
    let username_line = if app.applicant_team_username.is_empty() {
        String::new()
    } else {
        format!("@{}", app.applicant_team_username)
    };
    let submitted_at = format_when(app.submitted_at.unwrap_or(app.created_at));
    let snapshot_rows = build_snapshot_rows(&app);
    let field_count = app.form_snapshot.len();

    rsx! {
        div {
            class: "modal-backdrop sub-team-review-modal",
            "data-open": "true",
            role: "dialog",
            "aria-modal": "true",
            onclick: move |evt| {
                evt.stop_propagation();
                on_close.call(());
            },
            div {
                class: "modal review-modal",
                onclick: move |evt| evt.stop_propagation(),

                // Head: eyebrow + name + close X
                div { class: "review-modal__head",
                    div { class: "review-modal__icon",
                        lucide_dioxus::FileText { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "review-modal__title-wrap",
                        div { class: "review-modal__kicker", "{tr.review_modal_eyebrow}" }
                        div { class: "review-modal__title", "{display_name}" }
                        if !username_line.is_empty() {
                            div { class: "review-modal__handle", "{username_line}" }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "review-modal__close-x",
                        "aria-label": "{tr.review_modal_close}",
                        onclick: move |_| on_close.call(()),
                        lucide_dioxus::X { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                }

                // Body: meta strip + form snapshot table
                div { class: "review-modal__body",
                    div { class: "review-modal__meta",
                        div { class: "review-modal__meta-cell",
                            span { class: "review-modal__meta-label", "{tr.review_modal_submitted_at}" }
                            span { class: "review-modal__meta-value", "{submitted_at}" }
                        }
                        div { class: "review-modal__meta-cell",
                            span { class: "review-modal__meta-label", "{tr.queue_row_form_snapshot}" }
                            span { class: "review-modal__meta-value",
                                "{field_count} {tr.review_modal_fields_suffix}"
                            }
                        }
                    }
                    div { class: "review-modal__section-title", "{tr.review_modal_form_section}" }
                    div { class: "review-modal__snapshot",
                        if snapshot_rows.is_empty() {
                            div { class: "review-modal__empty", "{tr.empty_list}" }
                        }
                        for (key, value) in snapshot_rows.iter() {
                            div {
                                key: "{key}",
                                class: "review-modal__snapshot-row",
                                span { class: "review-modal__snapshot-key", "{key}" }
                                span { class: "review-modal__snapshot-value", "{value}" }
                            }
                        }
                    }
                }

                // Foot: close
                div { class: "review-modal__foot",
                    button {
                        r#type: "button",
                        class: "review-modal__close-btn",
                        onclick: move |_| on_close.call(()),
                        "{tr.review_modal_close}"
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum QueueRowMode {
    Idle,
    Approve,
    Return,
    Reject,
}

// ── Helpers ────────────────────────────────────────────────────────

fn short_id(id: &str) -> String {
    id.chars().take(8).collect()
}

fn team_initials(name: &str) -> String {
    name.split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase()
}

/// Cycles the mockup's avatar accent palette by a stable hash of the
/// application id so each row has a consistent color across renders.
fn avatar_variant_for(id: &str) -> &'static str {
    const VARIANTS: [&str; 4] = [
        "avatar--teal",
        "avatar--violet",
        "avatar--purple",
        "avatar--cyan",
    ];
    let h: u32 = id.bytes().fold(0u32, |a, b| a.wrapping_add(b as u32));
    VARIANTS[(h as usize) % VARIANTS.len()]
}

fn format_when(ms: i64) -> String {
    if ms <= 0 {
        return String::new();
    }
    use chrono::TimeZone;
    chrono::Utc
        .timestamp_millis_opt(ms)
        .single()
        .map(|t| t.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

/// Builds the snapshot key/value rows from the application's
/// `form_snapshot` (ordered, labeled) joined to its `form_values`.
/// Locked default fields sort to the top — same ordering as the apply
/// and child status pages.
fn build_snapshot_rows(app: &SubTeamApplicationResponse) -> Vec<(String, String)> {
    let mut snapshot: Vec<SubTeamFormFieldSnapshotDto> = app.form_snapshot.clone();
    snapshot.sort_by(|a, b| b.locked.cmp(&a.locked).then(a.order.cmp(&b.order)));
    snapshot
        .into_iter()
        .map(|field| {
            let raw = app
                .form_values
                .get(&field.field_id)
                .cloned()
                .unwrap_or_default();
            let value = match raw {
                serde_json::Value::String(s) => s,
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Array(a) => a
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
                    .join(", "),
                serde_json::Value::Null => String::new(),
                other => other.to_string(),
            };
            (field.label, value)
        })
        .collect()
}
