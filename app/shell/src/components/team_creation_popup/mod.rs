use crate::*;

translate! {
    TeamCreationPopupTranslate;

    create_team: {
        en: "Create Team",
        ko: "팀 생성",
    },

    nickname: {
        en: "Nickname",
        ko: "닉네임",
    },

    nickname_placeholder: {
        en: "Enter team nickname",
        ko: "팀 닉네임을 입력하세요",
    },

    username: {
        en: "Username",
        ko: "사용자명",
    },

    username_placeholder: {
        en: "Enter team username",
        ko: "팀 사용자명을 입력하세요",
    },

    description: {
        en: "Description",
        ko: "설명",
    },

    description_placeholder: {
        en: "Enter team description",
        ko: "팀 설명을 입력하세요",
    },

    cancel: {
        en: "Cancel",
        ko: "취소",
    },

    create: {
        en: "Create",
        ko: "생성",
    },

    username_error: {
        en: "Username must be at least 3 characters (lowercase letters, digits, underscores only)",
        ko: "사용자명은 3자 이상이어야 하며 소문자, 숫자, 밑줄만 사용할 수 있습니다",
    },
}

const DEFAULT_PROFILE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn TeamCreationPopup() -> Element {
    let tr: TeamCreationPopupTranslate = use_translate();
    let mut popup = use_popup();
    let mut team_ctx = use_team_context();
    let nav = use_navigator();

    let mut nickname = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut submitting = use_signal(|| false);

    let is_valid = use_memo(move || {
        let u = username.read();
        u.len() >= 3 && u.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    });

    rsx! {
        div { class: "flex flex-col gap-5 w-[400px] max-mobile:w-full",
            // Profile image preview
            div { class: "flex justify-center",
                img {
                    src: DEFAULT_PROFILE,
                    alt: "Team Profile",
                    class: "w-20 h-20 rounded-full object-cover",
                }
            }

            // Nickname input
            div { class: "flex flex-col gap-1.5",
                label { class: "text-sm font-medium text-c-secondary",
                    "{tr.nickname}"
                }
                input {
                    class: "w-full px-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary outline-none focus:border-primary",
                    r#type: "text",
                    placeholder: "{tr.nickname_placeholder}",
                    value: "{nickname}",
                    oninput: move |e| {
                        nickname.set(e.value());
                    },
                }
            }

            // Username input
            div { class: "flex flex-col gap-1.5",
                label { class: "text-sm font-medium text-c-secondary",
                    "{tr.username}"
                }
                div { class: "flex items-center gap-0",
                    span { class: "px-3 py-2 rounded-l-lg border border-r-0 border-divider bg-bg text-sm text-c-secondary",
                        "@"
                    }
                    input {
                        class: "w-full px-3 py-2 rounded-r-lg border border-divider bg-bg text-sm text-text-primary outline-none focus:border-primary",
                        r#type: "text",
                        placeholder: "{tr.username_placeholder}",
                        value: "{username}",
                        oninput: move |e| {
                            username.set(e.value().to_lowercase());
                            error_msg.set(None);
                        },
                    }
                }
                if !username.read().is_empty() && !is_valid() {
                    p { class: "text-xs text-red-500",
                        "{tr.username_error}"
                    }
                }
            }

            // Description input
            div { class: "flex flex-col gap-1.5",
                label { class: "text-sm font-medium text-c-secondary",
                    "{tr.description}"
                }
                textarea {
                    class: "w-full px-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary outline-none focus:border-primary resize-none",
                    rows: "3",
                    placeholder: "{tr.description_placeholder}",
                    value: "{description}",
                    oninput: move |e| {
                        description.set(e.value());
                    },
                }
            }

            // Error message
            if let Some(err) = error_msg.read().as_ref() {
                p { class: "text-xs text-red-500 text-center",
                    "{err}"
                }
            }

            // Buttons
            div { class: "flex gap-2.5 justify-end",
                button {
                    class: "px-4 py-2 rounded-lg border border-divider text-sm text-c-secondary hover:bg-hover cursor-pointer",
                    onclick: move |_| {
                        popup.close();
                    },
                    "{tr.cancel}"
                }
                button {
                    class: "px-4 py-2 rounded-lg bg-primary text-sm text-white hover:bg-primary/80 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed",
                    disabled: !is_valid() || nickname.read().is_empty() || submitting(),
                    onclick: move |_| {
                        let nickname_val = nickname.read().clone();
                        let username_val = username.read().clone();
                        let description_val = description.read().clone();

                        spawn(async move {
                            submitting.set(true);
                            error_msg.set(None);

                            let req = CreateTeamRequest {
                                username: username_val.clone(),
                                nickname: nickname_val,
                                profile_url: DEFAULT_PROFILE.to_string(),
                                description: description_val,
                            };

                            match create_team_handler(req).await {
                                Ok(_) => {
                                    // Reload teams
                                    if let Ok(teams) = get_user_teams_handler().await {
                                        team_ctx.set_teams(teams);
                                    }
                                    popup.close();
                                    nav.push(format!("/teams/{}", username_val));
                                }
                                Err(e) => {
                                    error_msg.set(Some(format!("{}", e)));
                                }
                            }
                            submitting.set(false);
                        });
                    },
                    "{tr.create}"
                }
            }
        }
    }
}
