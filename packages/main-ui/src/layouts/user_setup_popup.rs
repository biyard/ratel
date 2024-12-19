#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;

use crate::layouts::congraturation_popup::CongraturationPopup;

#[component]
pub fn UserSetupPopup(
    #[props(default ="user_setup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
) -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div {
            id,
            class,
            onclick: move |_| {
                popup.open(rsx!{
                    CongraturationPopup {}
                }).with_id("congraturation_popup").with_title("환영합니다!");
            },
            "UserSetupPopup page"
        }
    }
}
