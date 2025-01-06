#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::route::Language;
mod controller;

#[component]
pub fn PoliticianStatusPage(lang: Language) -> Element {
    rsx! {
        div {
            div { class: "flex flex-col justify-center items-center",
                div {
                    class: "text-3xl font-bold text-resultGrey",
                    style: "padding-bottom: 40px",
                    "Status"
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