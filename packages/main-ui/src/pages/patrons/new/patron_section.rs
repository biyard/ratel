#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn PatronSection(
    #[props(default ="patron_section".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    rsx! {
        div { id, class, "FeatureSection" }
    }
}
