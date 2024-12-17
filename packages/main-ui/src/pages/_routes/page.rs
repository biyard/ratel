#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    rsx! {
        div {
            // Header {}
            div { class: "flex flex-col justify-center items-center",
                div {
                    class: "text-3xl font-bold text-resultGrey",
                    style: "padding-bottom: 40px",
                    "Not Found"
                }
                div {
                    class: "text-xl font-normal text-resultGrey",
                    style: "padding-bottom: 40px",
                    "The Page you are looking for doesn't exists"
                }
            }
        }
    }
}
