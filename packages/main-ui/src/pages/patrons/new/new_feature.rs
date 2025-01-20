#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn NewFeature(
    #[props(default ="new_feature".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
) -> Element {
    rsx! {
        div { id, class, "NewFeature" }
    }
}
