#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use crate::components::icons;
use super::i18n::SignupPopupTranslate;

#[component]
pub fn SignupPopup(
    #[props(default ="signup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    onclick: EventHandler<Event<MouseData>>,
    lang: Language,
) -> Element {
    let tr = translate::<SignupPopupTranslate>(&lang);
    rsx! {
        div { id, class,
            div {
                class: "w-full flex flex-row my-[10px] p-[8px] bg-[#6D7AFF] rounded-[8px] justify-start items-center gap-[17px] cursor-pointer hover:bg-[#5C6BFF]",
                onclick,
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
        }
    }
}
