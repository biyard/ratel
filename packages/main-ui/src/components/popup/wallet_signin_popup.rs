#![allow(non_snake_case)]
use super::{
    loader_popup::LoaderPopup, signin_popup_footer::SigninPopupFooter, signup_popup::SignupPopup,
};
use crate::components::icons;
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn WalletSigninPopup(
    #[props(default ="wallet_signin_popup".to_string())] id: String,
    lang: Language,
    logo: Element,
    logo_origin: Element,
    msg: String,
) -> Element {
    let tr = translate::<WalletSigninPopupTranslate>(&lang);
    let mut popup: PopupService = use_context();
    let display_logo = logo.clone();
    rsx! {
        div { id, class: "w-full max-w-400 mx-5 max-mobile:!max-w-full",
            div { class: "flex flex-row justify-start gap-12",
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("backward button clicked");
                        popup.open(rsx! {
                            SignupPopup { lang }
                        });
                    },
                    span { class: "text-neutral-400 text-xs/14 font-medium",
                        icons::LeftArrow { color: "white", width: "24", height: "24" }
                    }
                }
                div { class: "justify-start text-white font-bold text-xl/24", {tr.title} }
            }
            div { class: "w-full flex justify-center items-center mt-35",
                // TODO: border-t rounded
                div { class: "w-[84px] h-[84px] bg-white rounded-full justify-center items-center flex",
                    div { class: "flex justify-center items-center", {display_logo} }
                }
            }
            div { class: "justify-center text-center text-neutral-400 text-base/24 mt-35 font-medium",
                "{tr.description}"
            }
            button {
                class: "w-full h-60 bg-primary rounded-[10px] mt-35 justify-center items-center cursor-pointer",
                onclick: move |_| {
                    let logo = logo.clone();
                    let logo_origin = logo_origin.clone();
                    let msg = msg.clone();
                    popup.open(rsx! {
                        LoaderPopup {
                            lang,
                            title: tr.title,
                            description: tr.description,
                            logo,
                            logo_origin,
                            msg,
                        }
                    });
                },
                span { class: "text-center text-black text-base font-bold", "{tr.title}" }
            }
            SigninPopupFooter { lang }
        }
    }
}

translate! {
    WalletSigninPopupTranslate;

    title: {
        ko: "로그인",
        en: "Sign in",
    },

    description: {
        ko: "지갑을 사용하여 로그인하려면 지갑에 메시지 요청을 서명하십시오",
        en: "Please sign the message request in your wallet to continue",
    },

    loader_message: {
        ko: "승인 대기 중",
        en: "Awaiting Confirmation",
    },
}
