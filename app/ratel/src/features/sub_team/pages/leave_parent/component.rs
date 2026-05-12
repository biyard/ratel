//! Leave-parent confirmation page. Mirrors
//! `assets/design/sub-team/child-leave-parent.html` — page header,
//! current-tie summary (child avatar → parent avatar), keep/lose
//! lists, optional message, confirm checkbox, and the danger action.
//!
//! Consumes `UseParentRelationship` for the parent info + the
//! `handle_leave_parent` mutation. URL `:username` is the **viewer's
//! own team** — the role check in `get_parent_relationship_handler`
//! resolves against that team.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::types::ParentRelationshipStatus;
use crate::features::sub_team::{
    use_parent_relationship, LeaveParentRequest, SubTeamTranslate, UseParentRelationship,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn TeamLeaveParentPage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_data = team_resource();
    let own_display = if team_data.nickname.is_empty() {
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
        SeoMeta { title: "{tr.leave_parent_title}" }
        LeaveParentForm {
            username: username.clone(),
            own_display: own_display.clone(),
            own_username: team_data.username.clone(),
        }
    }
}

#[component]
fn LeaveParentForm(username: String, own_display: String, own_username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let parent_rel = use_parent_relationship()?;
    let UseParentRelationship { relationship, .. } = parent_rel;

    let rel = relationship();
    let is_recognized = matches!(rel.status, ParentRelationshipStatus::RecognizedSubTeam);
    let parent_display = rel
        .parent_team_display_name
        .clone()
        .filter(|s| !s.is_empty())
        .or_else(|| rel.parent_team_id.clone())
        .unwrap_or_else(|| "—".to_string());
    let parent_username = rel.parent_team_username.clone().unwrap_or_default();
    let parent_initials = parent_display
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();
    let own_initials = own_display
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();
    let recognized_meta = rel.recognized_at.and_then(|ms| {
        if ms <= 0 {
            None
        } else {
            use chrono::TimeZone;
            chrono::Utc
                .timestamp_millis_opt(ms)
                .single()
                .map(|t| t.format("%Y-%m-%d").to_string())
        }
    });
    let parent_meta_line = match (parent_username.is_empty(), recognized_meta.as_ref()) {
        (false, Some(date)) => format!(
            "@{} · {} {} ~",
            parent_username, tr.leave_parent_tie_recognized_since, date
        ),
        (false, None) => format!("@{}", parent_username),
        (true, Some(date)) => format!(
            "{} {} ~",
            tr.leave_parent_tie_recognized_since, date
        ),
        (true, None) => String::new(),
    };

    let mut reason: Signal<String> = use_signal(String::new);
    let mut confirmed: Signal<bool> = use_signal(|| false);
    let ready = confirmed() && is_recognized;

    let username_for_back = username.clone();
    let username_for_after = username.clone();

    rsx! {
        // `.arena` would force height:100vh + overflow:hidden and clip
        // the page contents — leave is a stacked scroll page, not a
        // viewport-locked arena. Page-scoped scroll container per
        // `feedback_arena_page_scroll.md`.
        div { class: "sub-team-leave-parent",
            // ── Topbar ─────────────────────────────────────────────
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
                    button {
                        class: "brand-home",
                        "aria-label": "Home",
                        r#type: "button",
                        onclick: move |_| {
                            nav.push(Route::Index {});
                        },
                        lucide_dioxus::House { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    span { class: "brand-home__divider" }
                    div { class: "arena-topbar__logo arena-topbar__logo--child",
                        "{own_initials}"
                    }
                    div { class: "u-col",
                        span { class: "arena-topbar__title arena-topbar__title--child",
                            "{own_display}"
                        }
                        span { class: "arena-topbar__handle", "{tr.leave_parent_page_eyebrow}" }
                    }
                    span { class: "arena-topbar__status arena-topbar__status--caution",
                        "{tr.leave_parent_status_chip}"
                    }
                }
            }

            div { class: "page page--narrow",
                // ── Page header ────────────────────────────────────
                div { class: "page-header",
                    div { class: "page-header__main",
                        span { class: "page-header__eyebrow",
                            lucide_dioxus::LogOut { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.leave_parent_eyebrow_code}"
                        }
                        h1 { class: "page-header__title page-header__title--child",
                            "{tr.leave_parent_header_title_prefix}"
                            strong { "{parent_display}" }
                            "{tr.leave_parent_header_title_suffix}"
                        }
                        p { class: "page-header__sub", "{tr.leave_parent_header_sub}" }
                    }
                }

                if !is_recognized {
                    div { class: "notice notice--warn", "{tr.leave_parent_not_recognized}" }
                }

                // ── Current relationship tie ───────────────────────
                div { class: "current-tie",
                    div { class: "avatar avatar--teal avatar--lg", "{own_initials}" }
                    div { class: "tie-line",
                        span { class: "tie-dot tie-dot--child" }
                        span { class: "tie-dash" }
                        span { "{tr.leave_parent_tie_child_of}" }
                        span { class: "tie-dash" }
                        span { class: "tie-dot" }
                    }
                    div { class: "avatar avatar--lg", "{parent_initials}" }
                    div { class: "u-col tie-meta",
                        span { class: "tie-meta__name", "{parent_display}" }
                        if !parent_meta_line.is_empty() {
                            span { class: "tie-meta__sub", "{parent_meta_line}" }
                        }
                    }
                    span { class: "pill pill--approved", "Active" }
                }

                // ── Leave panel ────────────────────────────────────
                div { class: "leave",
                    div { class: "leave__head",
                        div { class: "leave__icon",
                            lucide_dioxus::LogOut { class: "w-5 h-5 [&>path]:stroke-current" }
                        }
                        div {
                            h2 { class: "leave__title", "{tr.leave_parent_title}" }
                            p { class: "leave__sub", "{tr.leave_parent_panel_sub}" }
                        }
                    }

                    div { class: "keep-list",
                        span { class: "keep-list__title", "{tr.leave_parent_keep_title}" }
                        div { class: "keep-list__item",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.leave_parent_keep_team}" }
                        }
                        div { class: "keep-list__item",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.leave_parent_keep_announcements}" }
                        }
                        div { class: "keep-list__item",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.leave_parent_keep_admins}" }
                        }
                    }

                    div { class: "lose-list",
                        span { class: "lose-list__title", "{tr.leave_parent_lose_title}" }
                        div { class: "lose-list__item",
                            lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.leave_parent_lose_broadcasts}" }
                        }
                        div { class: "lose-list__item",
                            lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.leave_parent_lose_dashboard}" }
                        }
                        div { class: "lose-list__item",
                            lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.leave_parent_lose_badge}" }
                        }
                        div { class: "lose-list__item",
                            lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                            span { "{tr.leave_parent_lose_reapply}" }
                        }
                    }

                    div { class: "field",
                        label { class: "field__label", "{tr.leave_parent_reason}" }
                        textarea {
                            class: "field__textarea",
                            id: "leave-reason",
                            "data-testid": "sub-team-leave-reason-input",
                            placeholder: "{tr.leave_parent_reason_placeholder}",
                            value: "{reason()}",
                            oninput: move |e| reason.set(e.value()),
                        }
                        span { class: "field__hint", "{tr.leave_parent_reason_hint}" }
                    }

                    div { class: "field",
                        label { class: "checkbox",
                            input {
                                r#type: "checkbox",
                                id: "confirm-check",
                                "data-testid": "sub-team-leave-confirm-check",
                                checked: confirmed(),
                                onchange: move |e| confirmed.set(e.checked()),
                            }
                            span { class: "checkbox__box",
                                lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            }
                            span { class: "checkbox__label",
                                "{tr.leave_parent_confirm_checkbox}"
                            }
                        }
                    }

                    div { class: "u-flex u-gap-10 u-justify-between leave__actions",
                        button {
                            class: "btn btn--ghost",
                            r#type: "button",
                            onclick: move |_| {
                                nav.go_back();
                            },
                            "{tr.cancel}"
                        }
                        button {
                            class: "btn btn--danger",
                            id: "confirm-btn",
                            "data-testid": "sub-team-leave-confirm-btn",
                            r#type: "button",
                            disabled: !ready,
                            // Previously `handle_leave_parent.call(req)` was
                            // fire-and-forget, immediately followed by
                            // `nav.push(...)`. The component unmount dropped
                            // the spawned task before the API request even
                            // left the browser, so the leave became a no-op.
                            // `.await` here keeps the task alive until the
                            // server responds; only then do we navigate.
                            //
                            // FIXME: `TeamArenaLayout`'s `use_context_provider`
                            // hooks only run on first mount, so SPA-navigating
                            // back to SocialIndex reuses the same layout
                            // instance and ParentHudPanel keeps showing the
                            // OLD "Recognized sub-team" state. Should be
                            // fixed by making the layout's context plumbing
                            // reactive (key-ed inner body remount or
                            // signal-sync on wall data change); for now the
                            // user must reload to see the standalone state.
                            onclick: {
                                let username = username_for_after.clone();
                                move |_| {
                                    let username = username.clone();
                                    async move {
                                        if !ready {
                                            return;
                                        }
                                        let raw = reason().clone();
                                        let req = LeaveParentRequest {
                                            reason: if raw.trim().is_empty() {
                                                None
                                            } else {
                                                Some(raw)
                                            },
                                        };
                                        let _ = parent_rel.leave(req).await;
                                        nav.replace(Route::SocialIndex { username });
                                    }
                                }
                            },
                            lucide_dioxus::LogOut { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.leave_parent_confirm}"
                        }
                    }
                }
            }
        }
    }
}

// Silence unused — `own_username` plumbed in case future server actions
// want it as a body param without re-resolving.
const _: fn(String) = |_| {};
