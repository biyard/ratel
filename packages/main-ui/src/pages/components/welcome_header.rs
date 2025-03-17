#![allow(non_snake_case)]
use crate::components::icons::Logo;
use bdk::prelude::*;

#[component]
pub fn WelcomeHeader(lang: Language, title: String, description: String) -> Element {
    rsx! {
        div { class: "justify-center text-center text-white font-bold text-2xl mt-35",
            "{title}"
        }
        div { class: "w-full flex justify-center items-center my-24",
            div { class: "w-100 h-100 justify-center items-center flex",
                // FIXME: Only logo image is supported
                div { class: "flex justify-center items-center", Logo {} }
            }
        }
        div { class: "justify-center text-center text-neutral-400 text-base font-medium",
            "{description}"
        }
    }
}
