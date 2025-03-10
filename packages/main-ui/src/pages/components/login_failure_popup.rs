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
    name: String,
    msg: String,
) -> Element {
    let tr = translate::<LoginFailurePopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();
    let message = format!("{} {}", tr.failure_message, name);
    rsx! {
        div { id, class,
            div { class: "justify-start text-white font-bold text-[20px] leading-[24px]",
                "{tr.title}"
            }
            div { class: "flex flex-col gap-[10px] mt-[35px]",
                div {
                    class: "w-full flex flex-row pl-[20px] py-[22px] bg-[#000203] border-[1px] rounded-[10px] justify-start items-center gap-[17px] cursor-pointer border-[#DB2780]",
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
                                tracing::error!("Failed to signup with Google");
                            }
                        };
                    },
                    {logo}
                    div { class: "flex flex-col gap-[3px]",
                        span { class: "text-white text-[16px] leading-[19px] font-semibold",
                            "{msg}"
                        }
                    }
                }

                div { class: "w-full flex flex-row pl-[20px] py-[10px] bg-[#DB27801A] rounded-[10px] justify-start items-center gap-[10px]",
                    icons::AlertCircle { color: "#DB2780" }
                    div { class: "flex flex-col gap-[3px]",
                        span { class: "text-[#DB2780] text-[15px] leading-[24px] font-semibold tracking-wider",
                            "{message}"
                        }
                        span { class: "text-[#DB2780] text-[15px] leading-[24px] font-semibold tracking-wider",
                            "{tr.try_again}"
                        }
                    }
                }
            }
            div { class: "flex flex-row gap-[10px] mt-[35px] justify-center",
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("Privacy policy clicked");
                    },
                    span { class: "text-[#C7C7C7] text-[12px] leading-[14px] font-medium",
                        "{tr.privacy_policy}"
                    }
                }
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("Privacy policy clicked");
                    },
                    span { class: "text-[#C7C7C7] text-[12px] leading-[14px] font-medium",
                        "{tr.term_of_service}"
                    }
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
