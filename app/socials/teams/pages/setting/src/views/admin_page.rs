use crate::components::{BasicInfoSection, DeleteTeamPopup, ProfileSection};
use crate::controllers::TeamResponse;
use crate::controllers::{UpdateTeamRequest, delete_team_handler, update_team_handler};
use crate::*;
use dioxus::prelude::*;
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn AdminPage(teamname: String, team: TeamResponse) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let mut popup = use_popup();
    let navigator = use_navigator();

    let mut team_state = use_signal(|| team);
    let mut is_editing = use_signal(|| false);
    let mut is_saving = use_signal(|| false);
    let mut message = use_signal(|| Option::<String>::None);

    let mut profile_url = use_signal(|| team_state().profile_url.clone().unwrap_or_default());
    let mut nickname = use_signal(|| team_state().nickname.clone());
    let mut html_contents = use_signal(|| team_state().html_contents.clone());

    let validation_nickname_required = tr.validation_nickname_required;
    let validation_description_min_length = tr.validation_description_min_length;
    let failed_update_team = tr.failed_update_team;

    let on_profile_upload = {
        let mut profile_url = profile_url.clone();
        move |url: String| {
            profile_url.set(url);
        }
    };

    let on_edit = {
        let mut is_editing = is_editing.clone();
        let mut profile_url = profile_url.clone();
        let mut nickname = nickname.clone();
        let mut html_contents = html_contents.clone();
        let team_state = team_state.clone();
        move |_evt: MouseEvent| {
            let current = team_state();
            profile_url.set(current.profile_url.clone().unwrap_or_default());
            nickname.set(current.nickname.clone());
            html_contents.set(current.html_contents.clone());
            is_editing.set(true);
        }
    };

    let on_cancel = {
        let mut is_editing = is_editing.clone();
        let mut profile_url = profile_url.clone();
        let mut nickname = nickname.clone();
        let mut html_contents = html_contents.clone();
        let team_state = team_state.clone();
        move |_evt: MouseEvent| {
            let current = team_state();
            profile_url.set(current.profile_url.clone().unwrap_or_default());
            nickname.set(current.nickname.clone());
            html_contents.set(current.html_contents.clone());
            is_editing.set(false);
        }
    };

    let on_save = {
        let mut is_saving = is_saving.clone();
        let mut is_editing = is_editing.clone();
        let mut message = message.clone();
        let mut team_state = team_state.clone();
        let teamname = teamname.clone();
        let profile_url = profile_url.clone();
        let nickname = nickname.clone();
        let html_contents = html_contents.clone();
        move |_evt: MouseEvent| {
            let display_name = nickname().trim().to_string();
            let description = html_contents().trim().to_string();
            let teamname = teamname.clone();

            if display_name.is_empty() {
                message.set(Some(validation_nickname_required.to_string()));
                return;
            }
            if !description.is_empty() && description.len() < 10 {
                message.set(Some(validation_description_min_length.to_string()));
                return;
            }

            spawn(async move {
                is_saving.set(true);
                message.set(None);

                let result = update_team_handler(
                    teamname,
                    UpdateTeamRequest {
                        nickname: Some(display_name.clone()),
                        description: Some(description.clone()),
                        profile_url: Some(profile_url()),
                        dao_address: None,
                    },
                )
                .await;

                is_saving.set(false);
                match result {
                    Ok(updated) => {
                        team_state.set(updated);
                        is_editing.set(false);
                    }
                    Err(err) => {
                        message.set(Some(format!("{}: {}", failed_update_team, err)));
                    }
                }
            });
        }
    };

    let on_cancel = {
        let mut popup = popup;
        move |_evt: MouseEvent| {
            popup.close();
            is_editing.set(false);
        }
    };
    let on_open_delete = {
        let mut popup = popup;
        let teamname = teamname.clone();
        let navigator = navigator.clone();
        move |_evt: MouseEvent| {
            let on_cancel = {
                let mut popup = popup;
                move |_evt: MouseEvent| {
                    popup.close();
                }
            };
            let on_confirm = {
                let mut popup = popup;
                let teamname = teamname.clone();
                let navigator = navigator.clone();
                move |_evt: MouseEvent| {
                    let mut popup = popup;
                    let teamname = teamname.clone();
                    let navigator = navigator.clone();
                    spawn(async move {
                        let result = delete_team_handler(teamname).await;
                        popup.close();
                        if result.is_ok() {
                            navigator.push("/");
                        } else if let Err(err) = result {
                            error!("Delete team failed: {}", err);
                        }
                    });
                }
            };
            popup
                .open(rsx! {
                    DeleteTeamPopup { on_confirm, on_cancel }
                })
                .without_backdrop_close();
        }
    };

    let permissions: TeamGroupPermissions = team_state().permissions.unwrap_or(0).into();
    let delete_team_permission = permissions.contains(TeamGroupPermission::TeamAdmin);

    rsx! {
        div { class: "w-full max-tablet:w-full flex flex-col gap-10 items-center",
            ProfileSection {
                profile_url: profile_url(),
                upload_logo_text: tr.upload_logo.to_string(),
                is_editing: is_editing(),
                on_profile_url_change: on_profile_upload,
            }

            div { class: "w-full flex flex-col gap-2.5",
                BasicInfoSection {
                    username: team_state().username.clone(),
                    nickname: nickname(),
                    html_contents: html_contents(),
                    on_nickname_change: {
                        let mut nickname = nickname.clone();
                        move |e: FormEvent| nickname.set(e.value())
                    },
                    on_description_change: {
                        let mut html_contents = html_contents.clone();
                        move |e: FormEvent| html_contents.set(e.value())
                    },
                    is_editing: is_editing(),
                }

                if !is_editing() {
                    div { class: "flex justify-end py-5 gap-2",
                        button {
                            class: "inline-flex items-center justify-center gap-2.5 whitespace-nowrap font-bold text-sm transition-all rounded-full bg-btn-primary-bg text-btn-primary-text border-btn-primary-outline hover:bg-btn-primary-hover-bg hover:border-btn-primary-hover-outline hover:text-btn-primary-hover-text px-5 py-2.5",
                            onclick: on_edit,
                            "{tr.edit}"
                        }
                        if delete_team_permission {
                            button {
                                class: "inline-flex items-center justify-center gap-2.5 whitespace-nowrap font-bold text-sm transition-all rounded-full bg-red-600 text-white hover:bg-red-600/90 px-5 py-2.5",
                                onclick: on_open_delete,
                                "{tr.delete}"
                            }
                        }
                    }
                } else {
                    div { class: "flex justify-end py-5 gap-2",
                        button {
                            class: "inline-flex items-center justify-center gap-2.5 whitespace-nowrap font-bold text-sm transition-all rounded-full bg-btn-outline-bg text-btn-outline-text border-btn-outline-outline hover:bg-btn-outline-hover-bg hover:border-btn-outline-hover-outline hover:text-btn-outline-hover-text px-5 py-2.5",
                            onclick: on_cancel,
                            "{tr.cancel}"
                        }
                        button {
                            class: "inline-flex items-center justify-center gap-2.5 whitespace-nowrap font-bold text-sm transition-all rounded-full bg-btn-primary-bg text-btn-primary-text border-btn-primary-outline hover:bg-btn-primary-hover-bg hover:border-btn-primary-hover-outline hover:text-btn-primary-hover-text px-5 py-2.5",
                            onclick: on_save,
                            disabled: is_saving(),
                            {if is_saving() { "Saving..." } else { tr.save_changes }}
                        }
                    }
                }

                if let Some(msg) = message() {
                    div { class: "text-sm text-text-secondary", "{msg}" }
                }
            }
        }
    }
}
