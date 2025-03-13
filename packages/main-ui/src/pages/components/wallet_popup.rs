#![allow(non_snake_case)]
use super::loader_popup::LoaderPopup;
use crate::{components::icons, services::user_service::UserService};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn WalletPopup(
    #[props(default ="wallet_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
) -> Element {
    let tr = translate::<WalletPopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();

    rsx! {
        div { id, class,
            div { class: "justify-start text-white font-bold text-xl/24", "{tr.title}" }
            div { class: "flex flex-col gap-10 mt-35",
                div {
                    class: "w-full flex flex-row pl-20 py-22 bg-black border-[1px] rounded-[10px] justify-start items-center gap-[17px] cursor-pointer hover:border-white",
                    style: if user_service.is_phantom_installed() { "cursor: pointer;" } else { "border: none; cursor: not-allowed;" },
                    onclick: move |_| async move {
                        if !user_service.is_phantom_installed() {
                            tracing::error!("Phantom wallet not installed");
                            return;
                        }
                        tracing::debug!("Signup with Phantom clicked");
                        user_service.set_signer_type("phantom");
                        popup.open(rsx! {
                            LoaderPopup {
                                class: "w-[400px] mx-auto",
                                lang,
                                logo: rsx! {
                                    icons::Phantom { width: "50", height: "50" }
                                },
                                logo_origin: rsx! {
                                    icons::Phantom {}
                                },
                                msg: tr.phantom,
                            }
                        });
                    },
                    icons::Phantom {}
                    div { class: "flex flex-col gap-3",
                        span {
                            class: "text-[16px] leading-[19px] font-semibold",
                            style: if user_service.is_phantom_installed() { "color: white;" } else { "color: #9F9F9F;" },
                            "{tr.phantom}"
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
    WalletPopupTranslate;

    title: {
        ko: "연결하기",
        en: "Connect",
    },

    phantom: {
        ko: "Phantom Wallet",
        en: "Phantom Wallet",
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
