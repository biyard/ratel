use crate::*;

#[component]
pub fn TeamCreationPopup() -> Element {
    let mut popup = use_popup();
    let mut team_ctx = use_team_context();
    let nav = use_navigator();
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut submitting = use_signal(|| false);
    let lang = use_language()();

    rsx! {
        crate::common::TeamCreationForm {
            submitting: submitting(),
            error_message: error_msg.read().clone(),
            on_cancel: move |_| {
                popup.close();
            },
            on_submit: move |payload| async move {
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
                    Ok(response) => {
                        let nav_username = username.clone();
                        team_ctx
                            .teams
                            .with_mut(move |teams| {
                                teams
                                    .push(TeamItem {
                                        pk: response.team_pk,
                                        username,
                                        nickname,
                                        profile_url,
                                        user_type: crate::common::types::UserType::Team,
                                        permissions: crate::features::posts::types::TeamGroupPermissions::all()
                                            .into(),
                                        description,
                                    });
                            });
                        debug!("Team created: {:?}", team_ctx.teams());
                        popup.close();
                        nav.push(Route::TeamHome { username: nav_username });
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("{}", e.translate(&lang))));
                    }
                }
                submitting.set(false);
            },
        }
    }
}
