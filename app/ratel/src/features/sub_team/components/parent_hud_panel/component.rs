//! Parent HUD icon + slide-out panel, embedded into a team home page.
//!
//! Mirrors `assets/design/sub-team/parent-home-with-button.html` —
//! one of three visual branches based on the team's parent
//! relationship status:
//!
//!   • RecognizedSubTeam → parent card (name · @handle · 인증 since)
//!     + actions (홈으로 이동 / 운영 수칙 보기 / 이탈).
//!   • PendingSubTeam → pending card + 신청 상태 확인 action.
//!   • Standalone → "독립 팀" empty state (no apply action — the user
//!     intentionally dropped it).
//!
//! Expects the current team's `TeamPartition` to already be provided
//! in context (the team arena layout installs one).

use crate::features::social::pages::team_arena::use_team_arena;
use crate::features::sub_team::types::ParentRelationshipStatus;
use crate::features::sub_team::{use_parent_relationship, SubTeamTranslate, UseParentRelationship};
use crate::route::Route;
use crate::*;

#[component]
pub fn ParentHudPanel() -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseParentRelationship {
        relationship,
        mut handle_cancel_application,
        ..
    } = use_parent_relationship()?;

    // Viewer is browsing their own team's home — the arena context
    // carries that team's username. Leave-parent navigates to
    // `/{own_username}/parent/leave` so the role check on the leave
    // page's `get_parent_relationship_handler` resolves against the
    // viewer's own admin team, not the parent (which would 401).
    let arena = use_team_arena();
    let own_username = (arena.username)();
    // Mutation actions (leave / edit application / cancel) are
    // admin-only; plain members see the panel for context but no
    // action buttons. The status-view link in the Pending branch is
    // read-only and stays visible for everyone.
    let is_admin = (arena.is_admin)();

    let rel = relationship();
    let status = rel.status;

    // Visibility policy:
    //   • Admin/Owner — icon visible in every status (Recognized /
    //     Pending / Standalone), action buttons gated inline below.
    //   • Plain member — icon ONLY when the team is fully Recognized.
    //     Pending / Standalone is admin-facing noise for them.
    if !is_admin && !matches!(status, ParentRelationshipStatus::RecognizedSubTeam) {
        return rsx! {};
    }

    let mut open: Signal<bool> = use_signal(|| false);

    let badge_state = match status {
        ParentRelationshipStatus::RecognizedSubTeam => "recognized",
        ParentRelationshipStatus::PendingSubTeam => "pending",
        ParentRelationshipStatus::Standalone => "none",
    };

    let parent_name = rel
        .parent_team_display_name
        .clone()
        .unwrap_or_else(|| "Parent team".to_string());
    let parent_username = rel.parent_team_username.clone().unwrap_or_default();
    let parent_initials = parent_name
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
    let username_for_home = parent_username.clone();
    let username_for_bylaws = parent_username.clone();
    let username_for_status = parent_username.clone();
    let username_for_edit = parent_username.clone();

    rsx! {
        div { class: "parent-panel-wrap",
            button {
                class: "hud-btn hud-btn--parent",
                "data-testid": "sub-team-hud-parent-btn",
                "data-state": "{badge_state}",
                "aria-label": "{tr.parent_icon}",
                "aria-expanded": "{open()}",
                r#type: "button",
                onclick: move |e| {
                    e.stop_propagation();
                    let v = !open();
                    open.set(v);
                },
                lucide_dioxus::GraduationCap { class: "w-5 h-5 [&>path]:stroke-current" }
            }
            div {
                class: "parent-panel",
                "data-open": "{open()}",
                role: "dialog",
                "aria-label": "Parent team",
                div { class: "parent-panel__head",
                    span { class: "parent-panel__title",
                        lucide_dioxus::GraduationCap { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.parent_panel_title}"
                    }
                    button {
                        class: "parent-panel__close",
                        "aria-label": "Close",
                        r#type: "button",
                        onclick: move |_| open.set(false),
                        lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                    }
                }

                match status {
                    ParentRelationshipStatus::RecognizedSubTeam => {
                        let meta_line = match &recognized_meta {
                            Some(date) => {
                                format!(
                                    "@{} · {} {} ~",
                                    parent_username,
                                    tr.parent_card_meta_recognized_since,
                                    date,
                                )
                            }
                            None => format!("@{}", parent_username),
                        };
                        let info_prefix = tr.parent_recognized_info_prefix.to_string();
                        let info_suffix = tr.parent_recognized_info_suffix.to_string();
                        let name_for_card = parent_name.clone();
                        let initials = parent_initials.clone();
                        rsx! {
                            div { class: "parent-state-block", "data-active": "true",
                                div { class: "parent-card",
                                    div { class: "parent-card__avatar", "{initials}" }
                                    div { class: "parent-card__body",
                                        div { class: "parent-card__name", "{name_for_card}" }
                                        div { class: "parent-card__meta", "{meta_line}" }
                                    }
                                    span { class: "parent-card__status parent-card__status--recognized",
                                        "{tr.parent_card_status_active}"
                                    }
                                }
                                div { class: "pp-info",
                                    "{info_prefix}"
                                    strong { "{name_for_card}" }
                                    "{info_suffix}"
                                }
                                div { class: "pp-actions",
                                    a {
                                        class: "pp-action",
                                        // FIXME: Doing a full-page navigation here because
                                        // TeamArenaLayout's `use_context_provider` hooks
                                        // only run on first mount. An SPA `nav.push` to a
                                        // different team's SocialIndex reuses the same
                                        // layout instance and keeps the OLD team's
                                        // TeamArenaContext + raw `TeamPartition`, so the
                                        // topbar and HUD render the previous team's data
                                        // even after the URL flips. Should be replaced
                                        // with a normal SPA nav once the layout's context
                                        // plumbing is made reactive (key-ed inner body
                                        // remount or signal-sync on wall data change).
                                        onclick: move |_| {
                                            if !username_for_home.is_empty() {
                                                let url = format!("/{}/", username_for_home);
                                                spawn(async move {
                                                    let _ = dioxus::document::eval(
                                                            &format!(
                                                                "window.location.assign({});",
                                                                serde_json::to_string(&url)
                                                                    .unwrap_or_else(|_| "\"/\"".to_string()),
                                                            ),
                                                        )
                                                        .await;
                                                });
                                            }
                                        },
                                        span { class: "pp-action__icon",
                                            lucide_dioxus::House { class: "w-3 h-3 [&>path]:stroke-current" }
                                        }
                                        span { class: "pp-action__body",
                                            span { class: "pp-action__title", "{tr.parent_action_open_home_title}" }
                                            span { class: "pp-action__sub", "{tr.parent_action_open_home_sub}" }
                                        }
                                        span { class: "pp-action__chev",
                                            lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                        }
                                    }
                                    a {
                                        class: "pp-action",
                                        onclick: move |_| {
                                            if !username_for_bylaws.is_empty() {
                                                nav.push(Route::TeamBylawsPage {
                                                    username: username_for_bylaws.clone(),
                                                });
                                            }
                                        },
                                        span { class: "pp-action__icon",
                                            lucide_dioxus::FileText { class: "w-3 h-3 [&>path]:stroke-current" }
                                        }
                                        span { class: "pp-action__body",
                                            span { class: "pp-action__title", "{tr.parent_action_view_bylaws_title}" }
                                            span { class: "pp-action__sub", "{tr.parent_action_view_bylaws_sub}" }
                                        }
                                        span { class: "pp-action__chev",
                                            lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                        }
                                    }
                                    if is_admin {
                                        a {
                                            class: "pp-action pp-action--danger",
                                            onclick: {
                                                let own_username = own_username.clone();
                                                move |_| {
                                                    // URL `:username` MUST be the viewer's own
                                                    // team — leave page's role check runs against
                                                    // whatever team the path resolves to. Passing
                                                    // the parent's username 401s because the
                                                    // viewer isn't an admin of the parent.
                                                    nav.push(Route::TeamLeaveParentPage {
                                                        username: own_username.clone(),
                                                    });
                                                }
                                            },
                                            span { class: "pp-action__icon",
                                                lucide_dioxus::LogOut { class: "w-3 h-3 [&>path]:stroke-current" }
                                            }
                                            span { class: "pp-action__body",
                                                span { class: "pp-action__title", "{tr.parent_action_leave_title}" }
                                                span { class: "pp-action__sub", "{tr.parent_action_leave_sub}" }
                                            }
                                            span { class: "pp-action__chev",
                                                lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ParentRelationshipStatus::PendingSubTeam => {
                        let name_for_card = parent_name.clone();
                        let initials = parent_initials.clone();
                        rsx! {
                            div { class: "parent-state-block", "data-active": "true",
                                div { class: "parent-card", "data-variant": "pending",
                                    div { class: "parent-card__avatar", "{initials}" }
                                    div { class: "parent-card__body",
                                        div { class: "parent-card__name", "{name_for_card}" }
                                        div { class: "parent-card__meta",
                                            if !parent_username.is_empty() {
                                                "@{parent_username}"
                                            }
                                        }
                                    }
                                    span { class: "parent-card__status parent-card__status--pending",
                                        "{tr.parent_card_status_pending}"
                                    }
                                }
                                div { class: "pp-info pp-info--amber", "{tr.parent_pending_info}" }
                                div { class: "pp-actions",
                                    a {
                                        class: "pp-action pp-action--primary",
                                        onclick: move |_| {
                                            if !username_for_status.is_empty() {
                                                nav.push(Route::TeamSubTeamApplicationStatusPage {
                                                    username: username_for_status.clone(),
                                                });
                                            }
                                        },
                                        span { class: "pp-action__icon",
                                            lucide_dioxus::Clock { class: "w-3 h-3 [&>path]:stroke-current" }
                                        }
                                        span { class: "pp-action__body",
                                            span { class: "pp-action__title", "{tr.parent_action_view_application_title}" }
                                            span { class: "pp-action__sub", "{tr.parent_action_view_application_sub}" }
                                        }
                                        span { class: "pp-action__chev",
                                            lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                        }
                                    }
                                    if is_admin {
                                        a {
                                            class: "pp-action",
                                            onclick: move |_| {
                                                if !username_for_edit.is_empty() {
                                                    nav.push(Route::TeamSubTeamApplyPage {
                                                        username: username_for_edit.clone(),
                                                    });
                                                }
                                            },
                                            span { class: "pp-action__icon",
                                                lucide_dioxus::Pencil { class: "w-3 h-3 [&>path]:stroke-current" }
                                            }
                                            span { class: "pp-action__body",
                                                span { class: "pp-action__title", "{tr.parent_action_edit_application_title}" }
                                                span { class: "pp-action__sub", "{tr.parent_action_edit_application_sub}" }
                                            }
                                            span { class: "pp-action__chev",
                                                lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                            }
                                        }
                                        a {
                                            class: "pp-action pp-action--danger",
                                            onclick: move |_| {
                                                handle_cancel_application.call();
                                                open.set(false);
                                            },
                                            span { class: "pp-action__icon",
                                                lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                                            }
                                            span { class: "pp-action__body",
                                                span { class: "pp-action__title", "{tr.parent_action_cancel_application_title}" }
                                                span { class: "pp-action__sub", "{tr.parent_action_cancel_application_sub}" }
                                            }
                                            span { class: "pp-action__chev",
                                                lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ParentRelationshipStatus::Standalone => rsx! {
                        div { class: "parent-state-block", "data-active": "true",
                            div { class: "pp-empty",
                                div { class: "pp-empty__icon",
                                    lucide_dioxus::Info { class: "w-4 h-4 [&>path]:stroke-current" }
                                }
                                div { class: "pp-empty__title", "{tr.parent_standalone_title}" }
                                div { class: "pp-empty__desc", "{tr.parent_standalone_desc}" }
                            }
                        }
                    },
                }
            }
        }
    }
}
