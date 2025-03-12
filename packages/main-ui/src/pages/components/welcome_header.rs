#![allow(non_snake_case)]
use crate::components::icons::Logo;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn WelcomeHeader(lang: Language, title: String, description: String) -> Element {
    rsx! {
        div { class: "justify-center text-white font-bold text-2xl", "{title}" }
        div { class: "w-full flex justify-center items-center my-24",
            div { class: "w-[100px] h-[100px] justify-center items-center flex",
                div { class: "flex justify-center items-center", Logo {} }
            }
        }
        div { class: "justify-center text-center text-neutral-400 text-base font-medium",
            "{description}"
        }
    }
}
