//! Parent HUD icon + slide-out panel, embedded into a team home page.
//!
//! Consumes `UseParentRelationship` to pick one of three visual states:
//! Recognized / PendingSubTeam / Standalone. The icon badge reflects the
//! state; clicking it opens a floating panel with the appropriate actions.
//!
//! Expects the current team's `TeamPartition` to already be provided in
//! context (the team arena layout installs one).

use crate::features::sub_team::types::ParentRelationshipStatus;
use crate::features::sub_team::{use_parent_relationship, SubTeamTranslate, UseParentRelationship};
use crate::route::Route;
use crate::*;

#[component]
pub fn ParentHudPanel(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseParentRelationship { relationship, .. } = use_parent_relationship()?;

    let status = relationship().status;

    let mut open: Signal<bool> = use_signal(|| false);
    let username_for_leave = username.clone();
    let username_for_status = username.clone();
    let username_for_apply = username.clone();

    let badge_state = match status {
        ParentRelationshipStatus::RecognizedSubTeam => "recognized",
        ParentRelationshipStatus::PendingSubTeam => "pending",
        ParentRelationshipStatus::Standalone => "none",
    };

    rsx! {
        div { class: "parent-panel-wrap",
            button {
                class: "hud-btn hud-btn--parent",
                id: "parent-btn",
                "data-state": "{badge_state}",
                "aria-label": "{tr.parent_icon}",
                "aria-expanded": "{open()}",
                onclick: move |e| {
                    e.stop_propagation();
                    let v = !open();
                    open.set(v);
                },
                lucide_dioxus::GraduationCap { class: "w-5 h-5 [&>path]:stroke-current" }
            }
            div {
                class: "parent-panel",
                id: "parent-panel",
                "data-open": "{open()}",
                role: "dialog",
                "aria-label": "Parent team",
                div { class: "parent-panel__head",
                    span { class: "parent-panel__title",
                        lucide_dioxus::GraduationCap { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.parent_icon}"
                    }
                    button {
                        class: "parent-panel__close",
                        "aria-label": "Close",
                        onclick: move |_| open.set(false),
                        lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                    }
                }

                match status {
                    ParentRelationshipStatus::RecognizedSubTeam => rsx! {
                        div { class: "parent-state-block", "data-active": "true",
                            div { class: "parent-card",
                                div { class: "parent-card__avatar", "P" }
                                div { class: "parent-card__body",
                                    div { class: "parent-card__name", "Parent team" }
                                    div { class: "parent-card__meta", "Recognized" }
                                }
                                span { class: "parent-card__status parent-card__status--recognized", "Active" }
                            }
                            div { class: "pp-actions",
                                a {
                                    class: "pp-action pp-action--danger",
                                    onclick: move |_| {
                                        nav.push(Route::TeamLeaveParentPage {
                                            username: username_for_leave.clone(),
                                        });
                                    },
                                    span { class: "pp-action__icon",
                                        lucide_dioxus::LogOut { class: "w-3 h-3 [&>path]:stroke-current" }
                                    }
                                    span { class: "pp-action__body",
                                        span { class: "pp-action__title", "{tr.leave_parent}" }
                                    }
                                    span { class: "pp-action__chev",
                                        lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                    }
                                }
                            }
                        }
                    },
                    ParentRelationshipStatus::PendingSubTeam => rsx! {
                        div { class: "parent-state-block", "data-active": "true",
                            div { class: "parent-card", "data-variant": "pending",
                                div { class: "parent-card__avatar", "P" }
                                div { class: "parent-card__body",
                                    div { class: "parent-card__name", "Parent team" }
                                    div { class: "parent-card__meta", "Pending review" }
                                }
                                span { class: "parent-card__status parent-card__status--pending", "Pending" }
                            }
                            div { class: "pp-actions",
                                a {
                                    class: "pp-action pp-action--primary",
                                    onclick: move |_| {
                                        nav.push(Route::TeamSubTeamApplicationStatusPage {
                                            username: username_for_status.clone(),
                                        });
                                    },
                                    span { class: "pp-action__icon",
                                        lucide_dioxus::Clock { class: "w-3 h-3 [&>path]:stroke-current" }
                                    }
                                    span { class: "pp-action__body",
                                        span { class: "pp-action__title", "{tr.application_status}" }
                                    }
                                    span { class: "pp-action__chev",
                                        lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                    }
                                }
                            }
                        }
                    },
                    ParentRelationshipStatus::Standalone => rsx! {
                        div { class: "parent-state-block", "data-active": "true",
                            div { class: "pp-empty",
                                div { class: "pp-empty__icon",
                                    lucide_dioxus::Info { class: "w-4 h-4 [&>path]:stroke-current" }
                                }
                                div { class: "pp-empty__title", "Standalone" }
                            }
                            div { class: "pp-actions",
                                a {
                                    class: "pp-action pp-action--primary",
                                    onclick: move |_| {
                                        nav.push(Route::TeamSubTeamApplyPage {
                                            username: username_for_apply.clone(),
                                        });
                                    },
                                    span { class: "pp-action__icon",
                                        lucide_dioxus::Plus { class: "w-3 h-3 [&>path]:stroke-current" }
                                    }
                                    span { class: "pp-action__body",
                                        span { class: "pp-action__title", "Apply to parent" }
                                    }
                                    span { class: "pp-action__chev",
                                        lucide_dioxus::ChevronRight { class: "w-3 h-3 [&>path]:stroke-current" }
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
