#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::icons;

#[component]
pub fn SignupPopup(
    #[props(default ="signup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    onclick: EventHandler<Event<MouseData>>,
) -> Element {
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
                        "Google로 계속하기"
                    }
                    span { class: "text-white text-[14px] leading-[13px] fond-regular",
                        "Quick Sign-in"
                    }
                }
            }
        }
    }
}
