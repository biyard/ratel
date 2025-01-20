#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::Language;

#[component]
pub fn FeatureSection(
    #[props(default ="feature_section".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
) -> Element {
    rsx! {
        div { id, class, "FeatureSection" }
    }
}
