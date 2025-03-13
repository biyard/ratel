#![allow(non_snake_case)]
use super::{login_failure_popup::LoginFailurePopup, user_setup_popup::UserSetupPopup};
use crate::services::user_service::{UserEvent, UserService};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn LoaderPopup(
    #[props(default ="loader_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
    logo: Element,
    logo_origin: Element,
    msg: String,
) -> Element {
    let tr = translate::<LoaderPopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();

    use_effect(move || {
        let logo = logo_origin.clone();
        let msg = msg.clone();
        spawn(async move {
            match user_service.login().await {
                UserEvent::Signup(principal, email, nickname, profile_url) => {
                    popup.open(rsx! {
                        UserSetupPopup {
                            class: "w-[400px] mx-[5px]",
                            nickname,
                            profile_url,
                            email,
                            principal,
                            lang,
                        }
                    });
                }
                UserEvent::Login => {
                    popup.close();
                }
                _ => {
                    tracing::error!("Failed to signup with Phantom");
                    popup
                        .open(rsx! {
                            LoginFailurePopup {
                                class: "w-[400px] mx-[5px]",
                                logo,
                                msg,
                                lang,
                            }
                        })
                        .with_id("login_failure_popup");
                }
            }
        });
    });

    rsx! {
        div { id, class,
            div { class: "justify-start text-white font-bold text-xl/24", "{tr.title}" }
            div { class: "w-full flex  justify-center items-center mt-[35px]",
                // TODO: border-t rounded
                div { class: "border-6 border-t-6 w-[82px] h-[82px] border-primary border-t-background rounded-full animate-spin" }
                div { class: "absolute w-[64px] h-[64px] bg-white rounded-full justify-center items-center",
                    {logo}
                }
            }
            div { class: "justify-center text-center text-white font-bold text-[16px] leading-[24px] mt-[35px]",
                "{tr.message}"
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
    LoaderPopupTranslate;

    title: {
        ko: "로그인",
        en: "Log in",
    },

    message: {
        ko: "팝업에서 계정에 로그인하세요",
        en: "Sign into your account in the pop-up",
    },

    privacy_policy: {
        ko: "개인정보 처리방침",
        en: "Privacy Policy",
    },

    term_of_service: {
        ko: "이용약관",
        en: "Term of Service",
    },
}
