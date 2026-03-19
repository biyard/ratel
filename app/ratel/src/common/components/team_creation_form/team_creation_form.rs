use crate::common::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TeamCreationPayload {
    pub profile_url: String,
    pub username: String,
    pub nickname: String,
    pub description: String,
}

translate! {
    TeamCreationFormTranslate;

    team_display_name: {
        en: "Team Name",
        ko: "팀 이름",
    },

    team_id: {
        en: "Team ID",
        ko: "팀 ID",
    },

    team_description: {
        en: "Team description",
        ko: "팀 설명을 입력해주세요.",
    },

    upload_logo: {
        en: "Upload logo",
        ko: "로고 업로드",
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
pub fn TeamCreationForm(
    on_submit: EventHandler<TeamCreationPayload>,
    #[props(optional)] on_cancel: Option<EventHandler<()>>,
    #[props(default)] submitting: bool,
    #[props(optional)] error_message: Option<String>,
) -> Element {
    let tr: TeamCreationFormTranslate = use_translate();
    let mut profile_url = use_signal(String::new);
    let mut nickname = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut description = use_signal(String::new);

    let username_validation = use_memo(move || {
        let u = username.read();
        if u.is_empty() {
            return (true, None);
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
        valid && !u.is_empty() && !n.is_empty() && !submitting
    });

    let preview_url = if profile_url.read().is_empty() {
        DEFAULT_PROFILE.to_string()
    } else {
        profile_url.read().clone()
    };

    rsx! {
        div { class: "w-100 max-tablet:w-full flex flex-col gap-10 items-center",
            FileUploader {
                on_upload_success: move |url| {
                    profile_url.set(url);
                },
                class: Some("group relative flex items-center justify-center size-40 max-mobile:size-20 mx-auto".to_string()),
                accept: Some("image/*".to_string()),
                img {
                    src: "{preview_url}",
                    alt: "logo",
                    class: "w-40 h-40 rounded-full object-cover cursor-pointer relative group max-mobile:size-20",
                }
                div { class: "absolute w-40 h-40 inset-0 bg-component-bg/50 flex items-center justify-center text-center rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-300 text-white font-semibold",
                    "{tr.upload_logo}"
                }
            }

            div { class: "flex flex-col gap-2.5 w-full",
                input {
                    class: "w-full px-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary light:text-neutral-900 outline-none focus:border-primary placeholder:text-c-secondary",
                    r#type: "text",
                    placeholder: "{tr.team_display_name}",
                    value: "{nickname}",
                    oninput: move |e| {
                        nickname.set(e.value());
                    },
                    "data-pw": "team-nickname-input",
                }
                div { class: "flex flex-col gap-0.25",
                    div { class: "relative",
                        span { class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-c-secondary",
                            "@"
                        }
                        input {
                            class: "w-full pl-8 pr-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary light:text-neutral-900 outline-none focus:border-primary placeholder:text-c-secondary",
                            r#type: "text",
                            placeholder: "{tr.team_id} (ex. ratel)",
                            value: "{username}",
                            oninput: move |e| {
                                username.set(e.value().to_lowercase());
                            },
                            "data-pw": "team-username-input",
                            aria_invalid: username_validation().0 == false,
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
                textarea {
                    class: "w-full px-3 py-2 rounded-lg border border-divider bg-bg text-sm text-text-primary light:text-neutral-900 outline-none focus:border-primary resize-none placeholder:text-c-secondary",
                    rows: "3",
                    placeholder: "{tr.team_description}",
                    value: "{description}",
                    oninput: move |e| {
                        description.set(e.value());
                    },
                    "data-pw": "team-description-input",
                }
            }

            if let Some(err) = error_message {
                p { class: "text-xs text-red-500 text-center", "{err}" }
            }

            div { class: "w-full grid grid-cols-2 gap-2.5",
                Button {
                    style: ButtonStyle::Secondary,
                    class: "w-full",
                    onclick: move |_| {
                        if let Some(handler) = &on_cancel {
                            handler.call(());
                        }
                    },
                    "{tr.cancel}"
                }
                Button {
                    style: ButtonStyle::Primary,
                    class: "w-full",
                    disabled: !can_submit(),
                    onclick: move |_| {
                        if !can_submit() {
                            return;
                        }
                        let profile_val = if profile_url.read().is_empty() {
                            DEFAULT_PROFILE.to_string()
                        } else {
                            profile_url.read().clone()
                        };
                        on_submit.call(TeamCreationPayload {
                            profile_url: profile_val,
                            username: username.read().clone(),
                            nickname: nickname.read().clone(),
                            description: description.read().clone(),
                        });
                    },
                    "{tr.create}"
                }
            }
        }
    }
}
