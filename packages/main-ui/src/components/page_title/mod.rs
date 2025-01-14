#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn PageTitle(title: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row justify-between text-xl font-semibold", "{title}" }
    }
}
