#![allow(non_snake_case)]
use super::user_setup_popup::UserSetupPopup;
use crate::{
    components::icons,
    services::user_service::{UserEvent, UserService},
};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

#[component]
pub fn LoginFailurePopup(
    #[props(default ="login_failure_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
    logo: Element,
    msg: String,
) -> Element {
    let tr = translate::<LoginFailurePopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();
    let service_name = user_service.get_signer_type();
    let message = format!("{} {}", tr.failure_message, service_name);
    rsx! {
        div { id, class,
            div { class: "justify-start text-white font-bold text-xl/24", "{tr.title}" }
            div { class: "flex flex-col gap-10 mt-35",
                div {
                    class: "w-full flex flex-row pl-20 py-22 bg-black border-[1px] rounded-[10px] justify-start items-center gap-[17px] cursor-pointer border-c-p-50",
                    onclick: move |_| async move {
                        match user_service.login().await {
                            UserEvent::Signup(principal, email, nickname, profile_url) => {
                                popup.open(rsx! {
                                    UserSetupPopup {
                                        class: "w-[400px] mx-[5px]",
                                        nickname,
                                        profile_url,
                                        email,
                                        principal,
                                        lang: lang.clone(),
                                    }
                                });
                            }
                            UserEvent::Login => {
                                popup.close();
                            }
                            _ => {
                                tracing::error!(
                                    "Failed to signup with {}", user_service.get_signer_type()
                                );
                            }
                        };
                    },
                    {logo}
                    div { class: "flex flex-col gap-3",
                        span { class: "text-white text-base/19 font-semibold", "{msg}" }
                    }
                }

                div { class: "w-full flex flex-row pl-[20px] py-[10px] bg-c-p-50-10 rounded-[10px] justify-start items-center gap-[10px]",
                    icons::AlertCircle { color: "#DB2780" }
                    div { class: "flex flex-col gap-3",
                        span { class: "text-c-p-50 text-[15px]/24 font-semibold tracking-wider",
                            "{message}"
                        }
                        span { class: "text-c-p-50 text-[15px]/24 font-semibold tracking-wider",
                            "{tr.try_again}"
                        }
                    }
                }
            }
            // TODO: applying policy and terms.
            div { class: "flex flex-row gap-10 mt-35 justify-center",
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("Privacy policy clicked");
                    },
                    span { class: "text-neutral-400 text-xs/14 font-medium", "{tr.privacy_policy}" }
                }
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("Privacy policy clicked");
                    },
                    span { class: "text-neutral-400 text-xs/14 font-medium", "{tr.term_of_service}" }
                }
            }
        }
    }
}

translate! {
    LoginFailurePopupTranslate;

    title: {
        ko: "로그인",
        en: "Log in",
    }

    failure_message: {
        ko: "로그인에 실패했습니다: ",
        en: "Failed to connect to ",
    },

    try_again: {
        ko: "다시 시도하시겠습니까?",
        en: "Would you like to try again?",
    }

    privacy_policy: {
        ko: "개인정보 처리방침",
        en: "Privacy Policy",
    },

    term_of_service: {
        ko: "이용약관",
        en: "Term of Service",
    },
}
