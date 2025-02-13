#![allow(non_snake_case)]
use super::i18n::WalletPopupTranslate;
use crate::components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn WalletPopup(
    #[props(default ="signup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    onclick: EventHandler<Event<MouseData>>,
    lang: Language,
) -> Element {
    let tr = translate::<WalletPopupTranslate>(&lang);
    rsx! {

        div { id, class, onclick,
            div { class: "rounded-[8px] bg-white w-[450ox] h-min-[198px] flex items-center justify-center",
                icons::MetaMask {}
            }
            div { class: "flex flex-col gap-[3px]",
                span { class: "text-white text-[16px] leading-[16px] font-extrabold",
                    "{tr.title}"
                }
                span { class: "text-white text-[14px] leading-[13px] fond-regular", "{tr.sub_text}" }
            }
        }
    }
}
