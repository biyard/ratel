#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::icons;

#[component]
pub fn LogoWrapper(#[props(default = "DemocraSee".to_string())] service_name: String) -> Element {
    rsx! {
        div { class: "flex flex-row items-center gap-[9px]",
            icons::Logo {}
            div { class: "text-[23px] font-extrabold text-white", "{service_name}" }
        }
    }
}
