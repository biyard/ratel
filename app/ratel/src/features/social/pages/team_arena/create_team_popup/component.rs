use crate::common::{use_popup, use_team_context, TeamCreationForm, TeamCreationPayload};
use crate::features::social::controllers::{create_team_handler, get_user_teams_handler, CreateTeamRequest};
use crate::route::Route;
use dioxus::prelude::*;

/// Arena-styled wrapper around the existing `TeamCreationForm`.
/// Reuses the form contents verbatim but hosts it inside a glass panel that
/// matches the team arena aesthetic (dark glass, gold accents, Orbitron).
#[component]
pub fn ArenaTeamCreationPopup() -> Element {
    let mut popup = use_popup();
    let mut team_ctx = use_team_context();
    let nav = use_navigator();
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut submitting = use_signal(|| false);

    let close_popup = move |_| popup.close();

    rsx! {

        div {
            class: "arena-create-team",
            "data-testid": "arena-create-team-popup",
            div { class: "arena-create-team__header",
                button {
                    class: "arena-create-team__close",
                    r#type: "button",
                    aria_label: "Close",
                    onclick: close_popup,
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "18",
                            y1: "6",
                            x2: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            y1: "6",
                            x2: "18",
                            y2: "18",
                        }
                    }
                }
                span { class: "arena-create-team__label", "New Squad" }
                span { class: "arena-create-team__title", "Create Team" }
                p { class: "arena-create-team__subhead",
                    "Spin up a new team and invite collaborators to the arena."
                }
            }

            div { class: "arena-create-team__body",
                TeamCreationForm {
                    submitting: submitting(),
                    error_message: error_msg.read().clone(),
                    on_cancel: move |_| {
                        popup.close();
                    },
                    on_submit: move |payload: TeamCreationPayload| async move {
                        submitting.set(true);
                        error_msg.set(None);

                        let TeamCreationPayload { profile_url, username, nickname, description } = payload;
                        let req = CreateTeamRequest {
                            username: username.clone(),
                            nickname: nickname.clone(),
                            profile_url: profile_url.clone(),
                            description: description.clone(),
                        };
                        match create_team_handler(req).await {
                            Ok(response) => {
                                if let Ok(resp) = get_user_teams_handler(None).await {
                                    team_ctx.set_teams(resp.items);
                                }
                                team_ctx.select_team(&response.team_pk);
                                popup.close();
                                nav.push(Route::TeamHome {
                                    username: username.clone(),
                                });
                            }
                            Err(e) => {
                                error_msg.set(Some(format!("{}", e)));
                            }
                        }
                        submitting.set(false);
                    },
                }
            }
        }
    }
}
