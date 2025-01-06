#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::route::Language;
use crate::theme::Theme;
mod controller;

#[component]
pub fn PoliticianStatusPage(lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-col justify-start w-full min-h-[100vh] text-white max-[1440px]:px-[10px] gap-[10px]",
            div {
                class: "text-xl font-semibold text-white",
                "Politicians"
            },
            PoliticianStatusTable {}
        }
    }
}

#[component]
pub fn PoliticianStatusTable() -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();

    rsx! {
        div { class: "w-full h-full flex flex-col bg-[{theme_data.primary06}] rounded-[8px] text-white",
            div { class: "w-full flex flex-row items-center gap-[90px] px-[15px] py-[10px]",
                div { class: "w-[280px]", 
                    span {
                        class: "text-xs font-semibold",
                        "NAME"
                    }
                }
                div { class: "text-xs font-semibold w-[150px]", "PARTY" }
                div { class: "text-xs font-semibold w-[200px]", "DISTRICT" }
                div { class: "text-xs font-semibold w-[210px]", "STANCE ON CRTPTO" }
                div { class: "text-xs font-semibold w-[210px]", "PROCLAIM" }
            }
            div { class: "w-full h-full flex flex-col gap-[10px] px-[20px] py-[10px]",
                PoliticianStatusRow {}
                PoliticianStatusRow {}
                PoliticianStatusRow {}
                PoliticianStatusRow {}
                PoliticianStatusRow {}
            }
        }
    }
}

#[component]
pub fn PoliticianStatusRow() -> Element {
    rsx! {
        div { class: "w-full h-[50px] flex flex-row items-center justify-start gap-[20px]",
            span { class: "text-[16px] font-semibold", "이름" }
            span { class: "text-[16px] font-semibold", "상태" }
            span { class: "text-[16px] font-semibold", "투표" }
            span { class: "text-[16px] font-semibold", "토론" }
            span { class: "text-[16px] font-semibold", "평가" }
        }
    }
}