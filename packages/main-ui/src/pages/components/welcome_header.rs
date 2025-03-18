#![allow(non_snake_case)]
use crate::components::icons::CharacterWithCircle;
use bdk::prelude::*;

#[component]
pub fn WelcomeHeader(lang: Language, title: String, description: String) -> Element {
    rsx! {
        div { class: "w-full flex flex-col gap-24 items-center justify-center mt-35",
            p { class: "text-white font-bold text-2xl", {title} }
            CharacterWithCircle { size: 100 }
            p { class: "text-neutral-400 text-center text-base font-medium", {description} }
        }
    }
}
