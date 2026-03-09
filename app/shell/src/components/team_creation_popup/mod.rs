use crate::*;
use crate::features::posts::types::TeamGroupPermissions;

#[component]
pub fn TeamCreationPopup() -> Element {
    let mut popup = use_popup();
    let team_ctx = use_team_context();
    let nav = use_navigator();
    let error_msg = use_signal(|| Option::<String>::None);
    let submitting = use_signal(|| false);

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
                        Ok(response) => {
                            let permissions: Vec<u8> = TeamGroupPermissions::all()
                                .0
                                .into_iter()
                                .map(|p| p as u8)
                                .collect();
                            let mut teams = team_ctx.teams.read().clone();
                            if let Some(existing) =
                                teams.iter_mut().find(|team| team.username == username)
                                .iter_mut()
                                .find(|team| team.username == username)
                            {
                                existing.nickname = nickname.clone();
                                existing.profile_url = profile_url.clone();
                                existing.description = description.clone();
                                existing.permissions = permissions.clone();
                                if existing.pk.is_empty() {
                                    existing.pk = response.team_pk.clone();
                                }
                            } else {
                                teams
                                    .push(crate::common::contexts::TeamItem {
                                        pk: response.team_pk.clone(),
                                        nickname: nickname.clone(),
                                        username: username.clone(),
                                        profile_url: profile_url.clone(),
                                        user_type: UserType::Team,
                                        permissions: permissions.clone(),
                                        description: description.clone(),
                                    });
                            }
                            let selected_index = teams
                                .iter()
                                .position(|team| team.username == username);
                            team_ctx.set_teams(teams);
                            if let Some(idx) = selected_index {
                                team_ctx.set_selected_index(idx);
                            }
                            if let Ok(teams) = get_user_teams_handler().await {
                                let selected_index = teams
                                    .iter()
                                    .position(|team| team.username == username);
                                team_ctx.set_teams(teams);
                                if let Some(idx) = selected_index {
                                    team_ctx.set_selected_index(idx);
                                }
                            }
                            popup.close();
                            nav.push(format!("/teams/{}/home", username));
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
