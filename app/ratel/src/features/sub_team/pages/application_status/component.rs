//! Child-side application status page. Mirrors
//! `assets/design/sub-team/child-application-status.html`.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::models::SubTeamApplicationStatus;
use crate::features::sub_team::{
    use_sub_team_application_status, ParentRelationshipStatusLabel,
    SubTeamApplicationResponse, SubTeamApplicationStatusLabel, SubTeamTranslate,
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
    let lang = dioxus_translate::use_language();
    let nav = use_navigator();

    let UseSubTeamApplicationStatus {
        relationship,
        mut applications,
        mut handle_cancel,
        ..
    } = use_sub_team_application_status()?;

    let rel = relationship();
    let rel_label: ParentRelationshipStatusLabel = rel.status.into();
    let rel_text = rel_label.translate(&lang());

    let items: Vec<SubTeamApplicationResponse> = applications.items().to_vec();
    let latest = items.first().cloned();

    let username_for_edit = username.clone();
    let username_for_back = username.clone();

    rsx! {
        div { class: "arena sub-team-application-status",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    a {
                        class: "back-btn",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.push(Route::TeamSubTeamApplyPage {
                                username: username_for_back.clone(),
                            });
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "{tr.status_page_eyebrow}" }
                        span { class: "topbar-title__main", "{team_display}" }
                    }
                }
            }

            div { class: "page page--narrow",
                // Relationship hero
                div { class: "status-hero", "data-status": "{rel.status:?}",
                    div { class: "status-hero__icon",
                        lucide_dioxus::Circle { class: "w-5 h-5 [&>path]:stroke-current" }
                    }
                    div { class: "status-hero__body",
                        span { class: "status-hero__eyebrow", "{tr.application_status}" }
                        h2 { class: "status-hero__title", "{rel_text}" }
                    }
                }

                // Latest application
                if let Some(app) = latest.clone() {
                    {
                        let status_label: SubTeamApplicationStatusLabel = app.status.into();
                        let status_text = status_label.translate(&lang());
                        let show_edit = matches!(
                            app.status,
                            SubTeamApplicationStatus::Returned,
                        );
                        let show_cancel = matches!(
                            app.status,
                            SubTeamApplicationStatus::Pending
                                | SubTeamApplicationStatus::Returned,
                        );
                        let app_id = app.id.clone();
                        let username_for_edit = username_for_edit.clone();
                        let decision_reason = app.decision_reason.clone();
                        rsx! {
                            section { class: "card",
                                div { class: "card__head",
                                    h3 { class: "card__title", "{tr.status_latest_application}" }
                                    span { class: "card__dash" }
                                    span { class: "pill pill--{app.status:?}", "{status_text}" }
                                }
                                if let Some(reason) = decision_reason {
                                    if !reason.is_empty() {
                                        div { class: "feedback",
                                            div { class: "feedback__title", "{tr.status_decision_reason}" }
                                            div { class: "feedback__text", "{reason}" }
                                        }
                                    }
                                }
                                div { class: "action-row",
                                    if show_edit {
                                        button {
                                            class: "btn btn--primary",
                                            onclick: move |_| {
                                                nav.push(Route::TeamSubTeamApplyPage {
                                                    username: username_for_edit.clone(),
                                                });
                                            },
                                            lucide_dioxus::Pencil { class: "w-3 h-3 [&>path]:stroke-current" }
                                            "{tr.status_edit_and_resubmit}"
                                        }
                                    }
                                    if show_cancel {
                                        button {
                                            class: "btn btn--ghost",
                                            onclick: move |_| {
                                                handle_cancel.call(app_id.clone());
                                            },
                                            lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                                            "{tr.status_cancel_application}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // History
                section { class: "card",
                    div { class: "card__head",
                        h3 { class: "card__title", "{tr.status_history}" }
                        span { class: "card__dash" }
                    }
                    if items.is_empty() {
                        div { class: "empty-row", "{tr.status_no_applications}" }
                    } else {
                        div { class: "history-list",
                            for app in items.iter() {
                                {
                                    let status_label: SubTeamApplicationStatusLabel = app.status.into();
                                    let status_text = status_label.translate(&lang());
                                    let parent_id = app.parent_team_id.clone();
                                    rsx! {
                                        div { key: "{app.id}", class: "history-row",
                                            div { class: "history-row__body",
                                                div { class: "history-row__title",
                                                    "→ "
                                                    "{parent_id}"
                                                }
                                                div { class: "history-row__meta",
                                                    "#{app.id}"
                                                }
                                            }
                                            span {
                                                class: "pill pill--{app.status:?}",
                                                "{status_text}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {applications.more_element()}
                }
            }
        }
    }
}
