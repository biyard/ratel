#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn CongraturationPopup(
    #[props(default ="congraturation_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
) -> Element {
    rsx! {
        div { id, class, "CongraturationPopup page"}
    }
}
