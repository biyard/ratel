#![allow(non_snake_case)]
use dioxus::prelude::*;

use super::components::*;

use dioxus_translate::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-center",
            Top { lang }
            About { lang }
            PoliticianStance { lang }
            Community { lang }
            Support { lang }
        }
    }
}
