//! Child-side application status page. Mirrors
//! `assets/design/sub-team/child-application-status.html` — status hero +
//! optional feedback card + submitted-answers snapshot + form-pin notice.
//! The mockup's tab switcher is intentionally not implemented (the live
//! page just renders the latest application's state).

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::models::SubTeamApplicationStatus;
use crate::features::sub_team::{
    use_sub_team_application_status, SubTeamApplicationResponse, SubTeamTranslate,
    UseSubTeamApplicationStatus,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn TeamSubTeamApplicationStatusPage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_data = team_resource();
    let team_display = if team_data.nickname.is_empty() {
        team_data.username.clone()
    } else {
        team_data.nickname.clone()
    };
    let team_pk = team_data.pk.clone();
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);

    rsx! {
        SeoMeta { title: "{tr.status_page_title}" }
        StatusBody { username: username.clone(), team_display: team_display.clone() }
    }
}

#[component]
fn StatusBody(username: String, team_display: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseSubTeamApplicationStatus { application, .. } = use_sub_team_application_status()?;

    let latest: Option<SubTeamApplicationResponse> = application();

    let username_for_edit = username.clone();
    let username_for_back = username.clone();

    rsx! {
        // `.arena` would force height:100vh + overflow:hidden and clip
        // the page contents — status is a stacked scroll page, not a
        // viewport-locked arena. Use the page-scoped scroll container
        // pattern from `feedback_arena_page_scroll.md`.
        div { class: "sub-team-application-status",
            // ── Topbar (back + brand + status pill) ─────────────────
            div { class: "arena-topbar",
                div { class: "arena-topbar__brand",
                    button {
                        class: "brand-home",
                        "aria-label": "Back",
                        r#type: "button",
                        onclick: move |_| {
                            nav.push(Route::SocialIndex {
                                username: username_for_back.clone(),
                            });
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    span { class: "brand-home__divider" }
                    div { class: "arena-topbar__logo arena-topbar__logo--child",
                        {team_display.chars().take(2).collect::<String>().to_uppercase()}
                    }
                    div { class: "u-col",
                        span { class: "arena-topbar__title arena-topbar__title--child",
                            "{team_display}"
                        }
                        span { class: "arena-topbar__handle", "{tr.status_page_eyebrow}" }
                    }
                    if let Some(app) = latest.as_ref() {
                        {
                            let (cls, label) = topbar_status_chip(app.status, &tr);
                            rsx! {
                                span { class: "arena-topbar__status {cls}", "{label}" }
                            }
                        }
                    }
                }
            }

            div { class: "page page--narrow",
                // Page header — eyebrow + title + sub. Mirrors the
                // mockup's `.page-header` block; tab switcher is omitted.
                div { class: "page-header",
                    div { class: "page-header__main",
                        span { class: "page-header__eyebrow",
                            lucide_dioxus::Clock { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.status_page_eyebrow}"
                        }
                        h1 { class: "page-header__title page-header__title--child",
                            "{team_display}"
                            strong { " · {tr.application_status}" }
                        }
                    }
                }

                if let Some(app) = latest.clone() {
                    {
                        let variant = status_variant(app.status);
                        let hero = hero_copy(app.status, &tr);
                        let when_text = format_relative_when(app.status, &app);
                        let field_count = app.form_snapshot.len();
                        let username_for_edit_cl = username_for_edit.clone();
                        let decision_reason = app
                            .decision_reason
                            .clone()
                            .unwrap_or_default();
                        let show_feedback = match app.status {
                            SubTeamApplicationStatus::Approved => !decision_reason.trim().is_empty(),
                            SubTeamApplicationStatus::Returned | SubTeamApplicationStatus::Rejected => {
                                true
                            }
                            _ => false,
                        };
                        let feedback_heading = feedback_heading(app.status, &tr); // Feedback card author = parent team. Falls back
                        let feedback_is_rejected = matches!(
                            app.status,
                            SubTeamApplicationStatus::Rejected
                        );
                        let parent_display = if app.parent_team_display_name.is_empty() {
                            app.parent_team_id.chars().take(8).collect::<String>()
                        } else {
                            app.parent_team_display_name.clone()
                        };
                        let parent_initials = parent_display
                            .split_whitespace() // ── Status hero ─────────────────────────
                            .take(2)
                            .filter_map(|w| w.chars().next())
                            .collect::<String>()
                            .to_uppercase();
                        let show_edit = matches!(app.status, SubTeamApplicationStatus::Returned);
                        let pill_class = pill_class_for(app.status);
                        let topbar_label = topbar_status_chip(app.status, &tr).1;
                        let snapshot_rows = build_snapshot_rows(&app);
                        let feedback_class = if feedback_is_rejected {
                            "feedback feedback--rejected"
                        } else {
                            "feedback"
                        };
                        rsx! {
                            // ── Status hero ─────────────────────────
                            div { class: "status-hero", "data-v": "{variant}",
                                div { class: "status-hero__icon", {hero_icon(app.status)} }
                                div {
                                    span { class: "status-hero__eyebrow", "{hero.eyebrow} · {when_text}" }
                                    h2 { class: "status-hero__title", "{hero.title}" }
                                    p { class: "status-hero__sub", "{hero.sub}" }
                                }
                                div { class: "u-col status-hero__meta",
                                    span { class: "pill {pill_class}", "{topbar_label}" }
                                    span { class: "status-hero__meta-foot",
                                        "{tr.status_form_version_meta} · {field_count} {tr.status_fields_count_suffix}"
                                    }
                                }
                            }

                            // ── Feedback card (returned / approved / rejected) ──
                            if show_feedback {
                                section { class: "card",
                                    div { class: "card__head",
                                        h3 { class: "card__title", "data-feedback-variant": "{variant}", "{feedback_heading}" }
                                        span { class: "card__dash" }
                                    }
                                    div { class: "{feedback_class}",
                                        div { class: "avatar feedback__avatar", "{parent_initials}" }
                                        div { class: "feedback__body",
                                            div { class: "feedback__head",
                                                span { class: "feedback__author", "{parent_display}" }
                                            }
                                            div { class: "feedback__title", "{feedback_heading}" }
                                            div { class: "feedback__text", "{decision_reason}" }
                                        }
                                    }
                                    div { class: "action-row",
                                        if show_edit {
                                            button {
                                                class: "btn btn--primary",
                                                r#type: "button",
                                                onclick: move |_| {
                                                    nav.push(Route::TeamSubTeamApplyPage {
                                                        username: username_for_edit_cl.clone(),
                                                    });
                                                },
                                                lucide_dioxus::Pencil { class: "w-3 h-3 [&>path]:stroke-current" }
                                                "{tr.status_edit_and_resubmit}"
                                            }
                                        }
                                    }
                                }
                            }

                            // ── Submitted-answers snapshot ──────────
                            section { class: "card",
                                div { class: "card__head",
                                    h3 { class: "card__title card__title--snapshot", "{tr.status_snapshot_heading}" }
                                    span { class: "card__dash" }
                                    span { class: "card__meta",
                                        "{tr.status_form_version_meta} · {field_count} {tr.status_fields_count_suffix}"
                                    }
                                }
                                div { class: "snapshot",
                                    if snapshot_rows.is_empty() {
                                        div { class: "empty-row", "{tr.status_no_applications}" }
                                    }
                                    for (key , value) in snapshot_rows.iter() {
                                        div { key: "{key}", class: "snapshot__row",
                                            span { class: "snapshot__key", "{key}" }
                                            span { class: "snapshot__value", "{value}" }
                                        }
                                    }
                                }
                            }

                            // ── Form-version pin notice ─────────────
                            div { class: "notice notice--teal",
                                div { class: "notice__icon",
                                    lucide_dioxus::Lock { class: "w-4 h-4 [&>path]:stroke-current" }
                                }
                                div { class: "notice__body",
                                    span { class: "notice__title", "{tr.status_form_pin_title}" }
                                    span { class: "notice__text", "{tr.status_form_pin_text}" }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "empty-row", "{tr.status_no_applications}" }
                }
            }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

struct HeroCopy {
    eyebrow: String,
    title: String,
    sub: String,
}

fn hero_copy(status: SubTeamApplicationStatus, tr: &SubTeamTranslate) -> HeroCopy {
    match status {
        SubTeamApplicationStatus::Pending => HeroCopy {
            eyebrow: tr.status_hero_pending_eyebrow.to_string(),
            title: tr.status_hero_pending_title.to_string(),
            sub: tr.status_hero_pending_sub.to_string(),
        },
        SubTeamApplicationStatus::Returned => HeroCopy {
            eyebrow: tr.status_hero_returned_eyebrow.to_string(),
            title: tr.status_hero_returned_title.to_string(),
            sub: tr.status_hero_returned_sub.to_string(),
        },
        SubTeamApplicationStatus::Approved => HeroCopy {
            eyebrow: tr.status_hero_approved_eyebrow.to_string(),
            title: tr.status_hero_approved_title.to_string(),
            sub: tr.status_hero_approved_sub.to_string(),
        },
        SubTeamApplicationStatus::Rejected => HeroCopy {
            eyebrow: tr.status_hero_rejected_eyebrow.to_string(),
            title: tr.status_hero_rejected_title.to_string(),
            sub: tr.status_hero_rejected_sub.to_string(),
        },
        _ => HeroCopy {
            eyebrow: tr.status_hero_pending_eyebrow.to_string(),
            title: tr.status_hero_pending_title.to_string(),
            sub: tr.status_hero_pending_sub.to_string(),
        },
    }
}

fn status_variant(status: SubTeamApplicationStatus) -> &'static str {
    match status {
        SubTeamApplicationStatus::Pending => "pending",
        SubTeamApplicationStatus::Returned => "returned",
        SubTeamApplicationStatus::Approved => "approved",
        SubTeamApplicationStatus::Rejected => "rejected",
        _ => "pending",
    }
}

fn pill_class_for(status: SubTeamApplicationStatus) -> &'static str {
    match status {
        SubTeamApplicationStatus::Pending => "pill--pending",
        SubTeamApplicationStatus::Returned => "pill--returned",
        SubTeamApplicationStatus::Approved => "pill--approved",
        SubTeamApplicationStatus::Rejected => "pill--rejected",
        _ => "pill--pending",
    }
}

fn topbar_status_chip(
    status: SubTeamApplicationStatus,
    _tr: &SubTeamTranslate,
) -> (&'static str, &'static str) {
    match status {
        SubTeamApplicationStatus::Pending => (
            "arena-topbar__status arena-topbar__status--pending",
            "Pending",
        ),
        SubTeamApplicationStatus::Returned => (
            "arena-topbar__status arena-topbar__status--returned",
            "Returned",
        ),
        SubTeamApplicationStatus::Approved => (
            "arena-topbar__status arena-topbar__status--approved",
            "Approved",
        ),
        SubTeamApplicationStatus::Rejected => (
            "arena-topbar__status arena-topbar__status--rejected",
            "Rejected",
        ),
        _ => (
            "arena-topbar__status arena-topbar__status--pending",
            "Pending",
        ),
    }
}

fn feedback_heading(status: SubTeamApplicationStatus, tr: &SubTeamTranslate) -> String {
    match status {
        SubTeamApplicationStatus::Returned => tr.status_feedback_returned_heading.to_string(),
        SubTeamApplicationStatus::Approved => tr.status_feedback_approved_heading.to_string(),
        SubTeamApplicationStatus::Rejected => tr.status_feedback_rejected_heading.to_string(),
        _ => String::new(),
    }
}

fn hero_icon(status: SubTeamApplicationStatus) -> Element {
    match status {
        SubTeamApplicationStatus::Pending => rsx! {
            lucide_dioxus::Clock { class: "w-7 h-7 [&>path]:stroke-current" }
        },
        SubTeamApplicationStatus::Returned => rsx! {
            lucide_dioxus::RotateCcw { class: "w-7 h-7 [&>path]:stroke-current" }
        },
        SubTeamApplicationStatus::Approved => rsx! {
            lucide_dioxus::Check { class: "w-7 h-7 [&>path]:stroke-current" }
        },
        SubTeamApplicationStatus::Rejected => rsx! {
            lucide_dioxus::X { class: "w-7 h-7 [&>path]:stroke-current" }
        },
        _ => rsx! {
            lucide_dioxus::Clock { class: "w-7 h-7 [&>path]:stroke-current" }
        },
    }
}

/// Builds the snapshot key/value rows from the application's
/// `form_snapshot` (ordered, labeled) joined to its `form_values`.
/// Locked default fields (`제안하는 팀 이름`, `설립 목적`) sort to the
/// top — same ordering as the apply page.
fn build_snapshot_rows(app: &SubTeamApplicationResponse) -> Vec<(String, String)> {
    let mut snapshot = app.form_snapshot.clone();
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

/// Picks the most meaningful timestamp for the hero eyebrow given the
/// application's status — submitted_at for pending/returned, decided_at
/// for approved/rejected — then formats it as `YYYY-MM-DD`.
fn format_relative_when(
    status: SubTeamApplicationStatus,
    app: &SubTeamApplicationResponse,
) -> String {
    let ms = match status {
        SubTeamApplicationStatus::Approved | SubTeamApplicationStatus::Rejected => {
            app.decided_at.unwrap_or(app.updated_at)
        }
        _ => app.submitted_at.unwrap_or(app.created_at),
    };
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
