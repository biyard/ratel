//! Leave-parent confirmation page. Mirrors
//! `assets/design/sub-team/child-leave-parent.html`. Consumes
//! `UseParentRelationship`.

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
        SeoMeta { title: "{tr.leave_parent_title}" }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        LeaveParentForm { username: username.clone(), team_display: team_display.clone() }
    }
}

#[component]
fn LeaveParentForm(username: String, team_display: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseParentRelationship {
        relationship,
        mut handle_leave_parent,
        ..
    } = use_parent_relationship()?;

    let rel = relationship();
    let is_recognized = matches!(rel.status, ParentRelationshipStatus::RecognizedSubTeam);
    let parent_id = rel
        .parent_team_id
        .clone()
        .unwrap_or_else(|| "—".to_string());

    let mut reason: Signal<String> = use_signal(String::new);
    let mut confirmed: Signal<bool> = use_signal(|| false);
    let ready = confirmed() && is_recognized;

    let username_for_back = username.clone();
    let username_for_after = username.clone();

    rsx! {
        div { class: "arena sub-team-leave-parent",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    a {
                        class: "back-btn",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.push(Route::TeamSubTeamApplicationStatusPage {
                                username: username_for_back.clone(),
                            });
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow",
                            "{tr.leave_parent_page_eyebrow}"
                        }
                        span { class: "topbar-title__main", "{team_display}" }
                    }
                }
            }

            div { class: "page page--narrow",
                if !is_recognized {
                    div { class: "notice notice--warn",
                        "{tr.leave_parent_not_recognized}"
                    }
                }

                div { class: "current-tie",
                    div { class: "current-tie__label", "{tr.leave_parent_current_tie}" }
                    div { class: "current-tie__value", "{parent_id}" }
                }

                div { class: "leave",
                    div { class: "leave__head",
                        div { class: "leave__icon",
                            lucide_dioxus::LogOut { class: "w-5 h-5 [&>path]:stroke-current" }
                        }
                        div {
                            h2 { class: "leave__title", "{tr.leave_parent_title}" }
                        }
                    }

                    div { class: "keep-list",
                        div { class: "keep-list__title", "{tr.leave_parent_keep_title}" }
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
                        div { class: "lose-list__title", "{tr.leave_parent_lose_title}" }
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
                        label { class: "field__label",
                            "{tr.leave_parent_reason}"
                        }
                        textarea {
                            class: "field__textarea",
                            id: "leave-reason",
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

                    div { class: "u-flex u-gap-10 u-justify-between",
                        button {
                            class: "btn btn--ghost",
                            onclick: move |_| {
                                nav.go_back();
                            },
                            "{tr.cancel}"
                        }
                        button {
                            class: "btn btn--danger",
                            id: "confirm-btn",
                            disabled: !ready,
                            onclick: move |_| {
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
                                handle_leave_parent.call(req);
                                nav.push(Route::TeamSubTeamApplicationStatusPage {
                                    username: username_for_after.clone(),
                                });
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
