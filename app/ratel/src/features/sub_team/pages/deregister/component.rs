//! Deregister sub-team confirmation page. Mirrors
//! `assets/design/sub-team/parent-deregister.html` — context summary,
//! consequence list, reason textarea, live notification preview, and
//! a confirm checkbox guarding the danger button.
//!
//! Consumes `UseSubTeamDeregister` (reason signal + handler) plus a
//! local loader for the sub-team's display data so the context card
//! can render its avatar + name without a manual fetch.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::controllers::get_sub_team_detail_handler;
use crate::features::sub_team::hooks::DeregParentUsername;
use crate::features::sub_team::{
    use_sub_team_deregister, DeregisterRequest, SubTeamTranslate, UseSubTeamDeregister,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn TeamSubTeamDeregisterPage(username: String, sub_team_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_data = team_resource();
    let parent_display_name = if team_data.nickname.is_empty() {
        team_data.username.clone()
    } else {
        team_data.nickname.clone()
    };
    let parent_username = team_data.username.clone();
    let team_pk = team_data.pk.clone();
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id.clone());
    let sub_team_id_for_ctx = sub_team_id.clone();
    use_context_provider(move || sub_team_id_for_ctx.clone());
    // Seed the parent username so `use_sub_team_deregister` can push the
    // management route on success instead of `nav.go_back()` (which lands
    // on the now-stale detail page).
    let parent_username_for_ctx = parent_username.clone();
    use_context_provider(move || DeregParentUsername(parent_username_for_ctx.clone()));

    rsx! {
        SeoMeta { title: "{tr.deregister_title}" }
        DeregisterForm {
            username: username.clone(),
            sub_team_id: sub_team_id.clone(),
            parent_display_name: parent_display_name.clone(),
            parent_username: parent_username.clone(),
        }
    }
}

#[component]
fn DeregisterForm(
    username: String,
    sub_team_id: String,
    parent_display_name: String,
    parent_username: String,
) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let UseSubTeamDeregister {
        team_id,
        sub_team_id: hook_sub_team_id,
        mut reason,
        mut handle_deregister,
    } = use_sub_team_deregister()?;

    // Load the sub-team's display data so the context card shows the
    // real avatar + name + handle. `get_sub_team_detail_handler` already
    // enforces the parent admin / sub-team link checks.
    let detail = use_loader(move || {
        let tid = team_id();
        let sid = hook_sub_team_id();
        async move { get_sub_team_detail_handler(tid, sid, None).await }
    })?;
    let detail_data = detail();
    let sub_team_display = detail_data.display_name.clone();
    let sub_team_username = detail_data.username.clone();
    let recognized_at = detail_data.recognized_at;

    let sub_team_initials = sub_team_display
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();
    let recognized_label = if recognized_at > 0 {
        use chrono::TimeZone;
        chrono::Utc
            .timestamp_millis_opt(recognized_at)
            .single()
            .map(|t| t.format("%Y-%m-%d").to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    let mut confirmed: Signal<bool> = use_signal(|| false);
    let reason_text = reason();
    let reason_trimmed = reason_text.trim().to_string();
    let preview_empty = reason_trimmed.is_empty();
    let preview_text = if preview_empty {
        tr.deregister_notif_preview_empty.to_string()
    } else {
        reason_trimmed.clone()
    };
    let ready = confirmed() && !reason_trimmed.is_empty();

    let _ = sub_team_id;
    let _ = username;

    let nav = use_navigator();

    rsx! {
        // `.arena` would force height:100vh + overflow:hidden and clip
        // the page contents. Page-scoped scroll container per
        // `feedback_arena_page_scroll.md`.
        div { class: "sub-team-deregister",
            // ── Topbar ─────────────────────────────────────────────
            div { class: "arena-topbar",
                div { class: "arena-topbar__brand",
                    button {
                        r#type: "button",
                        class: "brand-home",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    span { class: "brand-home__divider" }
                    div { class: "arena-topbar__logo arena-topbar__logo--parent",
                        {parent_display_name.chars().take(2).collect::<String>().to_uppercase()}
                    }
                    div { class: "u-col",
                        span { class: "arena-topbar__title arena-topbar__title--parent",
                            "{parent_display_name}"
                        }
                        span { class: "arena-topbar__handle", "{tr.deregister_eyebrow}" }
                    }
                    span { class: "arena-topbar__status arena-topbar__status--danger",
                        "{tr.deregister_status_chip}"
                    }
                }
            }

            div { class: "page page--narrow",
                // ── Context: sub-team being deregistered ───────────
                div { class: "context",
                    div { class: "avatar avatar--purple avatar--lg", "{sub_team_initials}" }
                    div { class: "context__body",
                        span { class: "context__name", "{sub_team_display}" }
                        if !sub_team_username.is_empty() {
                            span { class: "context__handle",
                                if recognized_label.is_empty() {
                                    "@{sub_team_username}"
                                } else {
                                    "@{sub_team_username} · {recognized_label} ~"
                                }
                            }
                        }
                    }
                    span { class: "pill pill--approved", "Recognized" }
                }

                // ── Deregister panel ───────────────────────────────
                div { class: "dereg",
                    div { class: "dereg__head",
                        div { class: "dereg__icon",
                            lucide_dioxus::TriangleAlert { class: "w-5 h-5 [&>path]:stroke-current" }
                        }
                        div {
                            h1 { class: "dereg__title",
                                "{tr.deregister_header_title_prefix}"
                                strong { "{sub_team_display}" }
                                "{tr.deregister_header_title_suffix}"
                            }
                            p { class: "dereg__sub", "{tr.deregister_header_sub}" }
                        }
                    }

                    // Consequences
                    div { class: "consequences",
                        span { class: "consequences__title", "{tr.deregister_consequences_title}" }
                        div { class: "consequences__item",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.deregister_consequence_unlink}" }
                        }
                        div { class: "consequences__item",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.deregister_consequence_notify}" }
                        }
                        div { class: "consequences__item",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.deregister_consequence_demote}" }
                        }
                        div { class: "consequences__item",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.deregister_consequence_content}" }
                        }
                        div { class: "consequences__item consequences__item--warn",
                            lucide_dioxus::Info { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.deregister_consequence_reapply}" }
                        }
                    }

                    // Reason
                    div { class: "field",
                        label { class: "field__label",
                            "{tr.deregister_reason} "
                            span { class: "req", "*" }
                        }
                        textarea {
                            class: "field__textarea",
                            id: "dereg-reason",
                            "data-testid": "sub-team-deregister-reason-input",
                            placeholder: "{tr.deregister_reason_placeholder}",
                            value: "{reason_text}",
                            oninput: move |e| reason.set(e.value()),
                        }
                        span { class: "field__hint", "{tr.deregister_reason_hint}" }
                    }

                    // Notification preview
                    div { class: "notif-preview",
                        span { class: "notif-preview__label", "{tr.deregister_notif_preview_label}" }
                        div { class: "notif-preview__body",
                            div { class: "notif-preview__avatar",
                                {parent_display_name.chars().take(2).collect::<String>().to_uppercase()}
                            }
                            div { class: "notif-preview__content",
                                div { class: "notif-preview__title",
                                    em { "{parent_display_name}" }
                                    "{tr.deregister_notif_preview_title_suffix}"
                                }
                                div { class: if preview_empty { "notif-preview__text notif-preview__text--placeholder" } else { "notif-preview__text" },
                                    "{preview_text}"
                                }
                            }
                        }
                    }

                    // Confirmation checkbox
                    div { class: "field",
                        label { class: "checkbox",
                            input {
                                r#type: "checkbox",
                                id: "confirm-check",
                                "data-testid": "sub-team-deregister-confirm-check",
                                checked: confirmed(),
                                onchange: move |e| confirmed.set(e.checked()),
                            }
                            span { class: "checkbox__box",
                                lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            }
                            span { class: "checkbox__label",
                                "{tr.deregister_confirm_check}"
                                small { class: "checkbox__hint", "{tr.deregister_confirm_check_hint}" }
                            }
                        }
                    }

                    div { class: "u-flex u-gap-10 u-justify-between dereg__actions",
                        div {
                            class: "btn btn--ghost",
                            role: "button",
                            onclick: move |_| {
                                nav.go_back();
                            },
                            "{tr.cancel}"
                        }
                        button {
                            class: "btn btn--danger",
                            id: "confirm-btn",
                            "data-testid": "sub-team-deregister-confirm-btn",
                            r#type: "button",
                            disabled: !ready,
                            onclick: move |_| {
                                if !ready {
                                    return;
                                }
                                handle_deregister
                                    .call(DeregisterRequest {
                                        reason: reason_trimmed.clone(),
                                    });
                            },
                            lucide_dioxus::LogOut { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.deregister_confirm}"
                        }
                    }
                }
            }
        }
    }
}

// Quiet warning for the now-unused nav variable when no other navigation
// stays on this page (cancel + back use anchor href).
const _: fn() = || {
    let _ = std::any::type_name::<Route>();
};
