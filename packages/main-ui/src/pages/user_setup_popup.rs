#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;
use dto::ServiceError;

use super::{congratulation_popup::CongratulationPopup, i18n::UserSetupPopupTranslate};
use crate::{components::checkbox::Checkbox, services::user_service::UserService, theme::Theme};

#[component]
pub fn UserSetupPopup(
    #[props(default ="user_setup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    nickname: String,
    profile_url: String,
    email: String,
    principal: String,
    lang: Language,
) -> Element {
    let mut popup: PopupService = use_context();
    let mut valid = use_signal(|| true);
    let mut nickname = use_signal(|| nickname.to_string());
    let mut agreed = use_signal(|| false);
    let user_service: UserService = use_context();
    let theme: Theme = use_context();
    let theme = theme.get_data();
    let btn_color = use_memo(move || {
        if agreed() {
            theme.primary100.clone()
        } else {
            theme.primary03.clone()
        }
    });
    let tr = translate::<UserSetupPopupTranslate>(&lang);

    rsx! {
        div { id, class,
            div { class: "flex flex-col items-start justify-start w-full pt-[10px] gap-[25px]",

                // Profile
                div { class: "w-full flex items-center justify-center",
                    img {
                        class: "w-[100px] h-[100px] rounded-[50%] object-contain",
                        src: "{profile_url}",
                    }
                }

                // Email
                if !email.is_empty() {
                    div { class: "flex flex-col gap-[8px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "이메일" }
                        }
                        div { class: "flex flex-col items-start w-full mt-[10px] gap-[8px]",
                            input {
                                class: "w-[400px] max-[400px]:w-[300px] h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] rounded-[4px] placeholder-[{theme.primary07}] rounded-[8px] text-[{theme.primary04}]",
                                value: "{email}",
                                disabled: true,
                            }
                        }
                    }
                }

                // Nickname
                div { class: "flex flex-col gap-[8px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "닉네임" }
                        span { class: "text-[14px] text-[#ff0000]", "*" }
                    }
                    div { class: "flex flex-col items-start w-full mt-[10px] gap-[8px]",
                        input {
                            class: "w-[400px] max-[400px]:w-[300px] h-[59px] px-[24px] py-[17.5px] bg-[{theme.background}] text-[18px] font-bold leading-[24px] rounded-[4px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: "{tr.enter_nickname}",
                            value: nickname(),
                            oninput: move |e| {
                                let value = e.value();
                                valid.set(value.chars().all(|c| c.is_alphanumeric()));
                                nickname.set(value);
                            },
                        }
                        if !valid() {
                            span { class: "text-[14px] font-bold leading-[24px] text-[{theme.primary04}]",
                                "{tr.special_characters}"
                            }
                        }
                    }
                }

                div { class: "flex flex-row gap-[10px] items-center",
                    Checkbox {
                        title: "{tr.agree_email}",
                        onchange: move |check| {
                            agreed.set(check);
                        },
                    }
                                // button { class: "px-[10px] py-[2px] rounded-[4px] bg-[{theme.primary11}] hover:bg-[{theme.primary05}]",
                //     div { class: "text-[14px] font-bold h-[24px] text-center text-white align-middle flex items-center justify-center",
                //         "자세히보기"
                //     }
                // }
                }

                button {
                    class: "w-full mt-[10px] rounded-[12px] bg-[{btn_color}] opacity-50 hover:opacity-100 text-[18px] font-extrabold leading-[24px] text-[{theme.primary05}] h-[59px] flex items-center justify-center",
                    onclick: move |_| {
                        if agreed() {
                            let nickname = nickname();
                            let principal = principal.clone();
                            let email = email.clone();
                            let profile_url = profile_url.clone();
                            spawn(async move {
                                if let Err(e) = user_service
                                    .login_or_signup(&principal, &email, &nickname, &profile_url)
                                    .await
                                {
                                    match e {
                                        ServiceError::UserAlreadyExists => {
                                            popup.close();
                                            return;
                                        }
                                        e => {
                                            tracing::error!("UserSetupPopup::signup: error={:?}", e);
                                        }
                                    }
                                } else {
                                    tracing::debug!("UserSetupPopup::signup: success");
                                    popup
                                        .open(rsx! {
                                            CongratulationPopup { lang: lang.clone() }
                                        })
                                        .with_id("congratulation_popup")
                                        .with_title(tr.welcome)
                                        .without_close();
                                }
                            });
                        }
                    },
                    "{tr.next}"
                }
            }
        }
    }
}
