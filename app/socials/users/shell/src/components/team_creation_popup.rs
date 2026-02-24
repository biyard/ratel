use crate::controllers::{CreateTeamRequest, create_team_handler, get_user_teams_handler};
use crate::*;

translate! {
    TeamCreationPopupTranslate;

    create_new_team: {
        en: "Create New Team",
        ko: "새 팀 생성",
    },

    team_display_name: {
        en: "Team Display Name",
        ko: "팀 닉네임",
    },

    team_display_name_placeholder: {
        en: "Enter team display name",
        ko: "팀 닉네임을 입력하세요",
    },

    team_id: {
        en: "Team ID",
        ko: "팀 ID",
    },

    team_id_placeholder: {
        en: "Team ID (e.g. ratel)",
        ko: "팀 ID (예: ratel)",
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

    username_too_short: {
        en: "Team ID must be at least 3 characters",
        ko: "팀 ID는 3자 이상이어야 합니다",
    },

    username_invalid_chars: {
        en: "Only lowercase letters, digits, and underscores allowed",
        ko: "소문자, 숫자, 밑줄만 사용할 수 있습니다",
    },
}

const DEFAULT_PROFILE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn TeamCreationPopup() -> Element {
    let tr: TeamCreationPopupTranslate = use_translate();
    let mut popup = use_popup();
    let mut team_ctx = common::contexts::use_team_context();
    let nav = use_navigator();

    let mut nickname = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut submitting = use_signal(|| false);

    let username_validation = use_memo(move || {
        let u = username.read();
        if u.is_empty() {
            return (true, None); // empty is "valid" (no error shown yet)
        }
        if u.len() < 3 {
            return (false, Some(tr.username_too_short));
        }
        if !u
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            return (false, Some(tr.username_invalid_chars));
        }
        (true, None)
    });

    let can_submit = use_memo(move || {
        let (valid, _) = username_validation();
        let u = username.read();
        let n = nickname.read();
        valid && !u.is_empty() && !n.is_empty() && !submitting()
    });

    rsx! {
        div { class: "flex flex-col gap-5 w-[400px] max-mobile:w-full",
            // Profile image preview
            div { class: "flex justify-center",
                div { class: "relative group",
                    img {
                        src: DEFAULT_PROFILE,
                        alt: "Team Profile",
                        class: "w-20 h-20 rounded-full object-cover",
                    }
                }
            }

            // Nickname (display name) input
            div { class: "flex flex-col gap-1.5",
                input {
                    class: "w-full px-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary outline-none focus:border-primary placeholder:text-c-secondary",
                    r#type: "text",
                    placeholder: "{tr.team_display_name_placeholder}",
                    value: "{nickname}",
                    oninput: move |e| {
                        nickname.set(e.value());
                    },
                }
            }

            // Username (team ID) input with @ prefix
            div { class: "flex flex-col gap-1",
                div { class: "relative",
                    span { class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-sm text-c-secondary",
                        "@"
                    }
                    input {
                        class: "w-full pl-8 pr-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary outline-none focus:border-primary placeholder:text-c-secondary",
                        r#type: "text",
                        placeholder: "{tr.team_id_placeholder}",
                        value: "{username}",
                        oninput: move |e| {
                            username.set(e.value().to_lowercase());
                            error_msg.set(None);
                        },
                    }
                }
                {
                    let (_, validation_error) = username_validation();
                    match validation_error {
                        Some(err) => rsx! {
                            p { class: "text-xs text-red-500", "{err}" }
                        },
                        None => rsx! {},
                    }
                }
            }

            // Description textarea
            div { class: "flex flex-col gap-1.5",
                textarea {
                    class: "w-full px-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary outline-none focus:border-primary resize-none placeholder:text-c-secondary",
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
                p { class: "text-xs text-red-500 text-center", "{err}" }
            }

            // Action buttons
            div { class: "grid grid-cols-2 gap-2.5",
                button {
                    class: "w-full px-4 py-2 rounded-full border border-divider text-sm text-c-secondary hover:bg-hover cursor-pointer",
                    onclick: move |_| {
                        popup.close();
                    },
                    "{tr.cancel}"
                }
                button {
                    class: if can_submit() { "w-full px-4 py-2 rounded-full bg-primary text-sm text-white hover:bg-primary/80 cursor-pointer" } else { "w-full px-4 py-2 rounded-full bg-neutral-600 text-sm text-white cursor-not-allowed" },
                    disabled: !can_submit(),
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
                                    // Reload teams into context
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
