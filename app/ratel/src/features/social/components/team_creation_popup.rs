use crate::features::social::controllers::{create_team_handler, CreateTeamRequest};
use crate::features::social::*;

#[component]
pub fn TeamCreationPopup() -> Element {
    let mut popup = use_popup();
    let team_ctx = use_team_context();
    let nav = use_navigator();
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut submitting = use_signal(|| false);

    rsx! {
        crate::common::TeamCreationForm {
            submitting: submitting(),
            error_message: error_msg.read().clone(),
            on_cancel: move |_| {
                popup.close();
            },
            on_submit: move |payload| {
                let mut popup = popup.clone();
                let mut team_ctx = team_ctx.clone();
                let nav = nav.clone();
                let mut error_msg = error_msg.clone();
                let mut submitting = submitting.clone();

                spawn(async move {
                    submitting.set(true);
                    error_msg.set(None);

                    let crate::common::TeamCreationPayload {
                        profile_url,
                        username,
                        nickname,
                        description,
                    } = payload;

                    let req = CreateTeamRequest {
                        username: username.clone(),
                        nickname: nickname.clone(),
                        profile_url: profile_url.clone(),
                        description: description.clone(),
                    };

                    match create_team_handler(req).await {
                        Ok(_response) => {
                            team_ctx.teams.restart();
                            popup.close();
                            nav.push(format!("/{}/home", username));
                        }
                        Err(e) => {
                            error_msg.set(Some(format!("{}", e)));
                        }
                    }
                    submitting.set(false);
                });
            },
        }
    }
}
