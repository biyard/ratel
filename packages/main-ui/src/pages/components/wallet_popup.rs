#![allow(non_snake_case)]
use super::{loader_popup::LoaderPopup, signin_popup_footer::SigninPopupFooter};
use crate::{components::icons, services::user_service::UserService};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

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
                                title: tr.phantom,
                                description: tr.loader_message,
                                logo: rsx! {
                                    icons::Phantom { width: "50", height: "50" }
                                },
                                logo_origin: rsx! {
                                    icons::Phantom {}
                                },
                                msg: tr.phantom,
                            }
                        }).with_id("loader_popup");
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
            SigninPopupFooter { lang }
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

    loader_message: {
        ko: "승인 대기 중",
        en: "Awaiting Confirmation",
    },
}
