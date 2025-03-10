#![allow(non_snake_case)]
use super::{i18n::SignupPopupTranslate, user_setup_popup::UserSetupPopup};
use crate::{
    components::icons,
    services::user_service::{UserEvent, UserService},
};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

#[component]
pub fn SignupPopup(
    #[props(default ="signup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
) -> Element {
    let tr = translate::<SignupPopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();

    rsx! {
        div { id, class,
            div {
                class: "w-full flex flex-row my-[10px] p-[8px] bg-[#6D7AFF] rounded-[8px] justify-start items-center gap-[17px] cursor-pointer hover:bg-[#5C6BFF]",
                onclick: move |_| async move {
                    tracing::debug!("Signup with Google clicked");
                    user_service.set_signer_type("google");
                    match user_service.login().await {
                        UserEvent::Signup(principal, email, nickname, profile_url) => {
                            popup.open(rsx! {
                                UserSetupPopup {
                                    class: "w-[400px]",
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
                            popup.close();
                        }
                    };
                },
                div { class: "rounded-[8px] bg-white w-[62px] h-[62px] flex items-center justify-center",
                    icons::Google {}
                }
                div { class: "flex flex-col gap-[3px]",
                    span { class: "text-white text-[16px] leading-[16px] font-extrabold",
                        "{tr.continue_with_google}"
                    }
                    span { class: "text-white text-[14px] leading-[13px] fond-regular",
                        "{tr.quick_sign_in}"
                    }
                }
            }

            div {
                class: "w-full flex flex-row my-[10px] p-[8px] bg-[#AB9FF1] rounded-[8px] justify-start items-center gap-[17px] cursor-pointer hover:bg-[#9A8EFF]",
                style: if user_service.is_phantom_installed() { "background: #AB9FF1; cursor: pointer;" } else { "background: #9F9F9F; cursor: not-allowed;" },
                onclick: move |_| async move {
                    if !user_service.is_phantom_installed() {
                        tracing::error!("Phantom wallet not installed");
                        return;
                    }
                    tracing::debug!("Signup with Phantom clicked");
                    user_service.set_signer_type("phantom");
                    match user_service.login().await {
                        UserEvent::Signup(principal, email, nickname, profile_url) => {
                            popup.open(rsx! {
                                UserSetupPopup {
                                    class: "w-[400px]",
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
                            tracing::error!("Failed to signup with Phantom");
                            popup.close();
                        }
                    };
                },
                div { class: "rounded-[8px] w-[62px] h-[62px] flex items-center justify-center",
                    icons::Phantom {}
                }
                div { class: "flex flex-col gap-[3px]",
                    span { class: "text-white text-[16px] leading-[16px] font-extrabold",
                        "{tr.continue_with_phantom_wallet}"
                    }
                    span { class: "text-white text-[14px] leading-[13px] fond-regular",
                        if user_service.is_phantom_installed() {
                            "{tr.connect_wallet}"
                        } else {
                            "{tr.need_wallet}"
                        }
                    }
                }
            }
        }
    }
}
