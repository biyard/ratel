#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::route::Language;
use crate::theme::Theme;
use crate::components::icons;

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
            div { class: "w-full flex flex-row items-center gap-[90px] px-[15px] py-[10px] border-b-[1px] border-[#323342]", 
                div { class: "flex items-center w-[280px] gap-[2px]", 
                    span {
                        class: "text-xs font-semibold",
                        "NAME",
                    }
                    icons::Search { color: "white" }
                }
                div { class: "flex items-center w-[150px] gap-[2px]", 
                    span {
                        class: "text-xs font-semibold",
                        "PARTY",
                    }
                    icons::Sort { color: "white", filled: false }
                }
                div { class: "flex items-center w-[200px] gap-[2px]", 
                    span {
                        class: "text-xs font-semibold",
                        "DISTRICT",
                    }
                    icons::Sort { color: "white", filled: true }
                }
                div { class: "flex items-center w-[210px] gap-[2px]", 
                    span {
                        class: "text-xs font-semibold",
                        "STANCE ON CRTPTO",
                    }
                    icons::Sort { color: "white", filled: false }
                }
                div { class: "flex items-start w-[210px] gap-[1px]", 
                    span {
                        class: "text-xs font-semibold",
                        "PROCLAIM",
                    }
                    icons::Tooltip { color: "{theme_data.primary00}" }
                }
            }
            div { class: "w-full h-full flex flex-col gap-[10px] px-[15px] py-[10px]",
                PoliticianStatusRow { stance: "supportive" }
                PoliticianStatusRow { stance: "against" }
                PoliticianStatusRow { stance: "neutral" }
                PoliticianStatusRow { stance: "a" }
                PoliticianStatusRow {}
            }
        }
    }
}

#[component]
pub fn PoliticianStatusRow(
    #[props(default = "None".to_string())] name: String,
    #[props(default = "None".to_string())] party: String,
    #[props(default = "None".to_string())] district: String,
    #[props(default = None)] stance: Option<String>,
) -> Element {
    let theme: Theme = use_context();
    let theme_data: crate::theme::ThemeData = theme.get_data();

    rsx! {
        div { class: "w-full h-[50px] flex flex-row items-center justify-start gap-[90px]",
            div { class: "w-[280px]",
                span { class: "text-xs font-semibold", "{name}" }
            }
            div { class: "text-sm w-[150px]", "{party}" }
            div { class: "text-sm w-[200px]", "{district}" }
            div { class: "flex items-center text-sm w-[210px] gap-[10px]", 
                if let Some(stance) = stance {
                    if stance == "supportive" {
                        icons::Pros { color: "{theme_data.active00}" },
                        "Supportive"
                    } else if stance == "against" {
                        icons::Cons { color: "{theme_data.active_false}" },
                        "Against"
                    } else {
                        icons::HandPalm { color: "#E7D8C6" },
                        "Neutral"
                    }
                } else {
                    "No Stance"
                }
            }
            div { class: "px-[10px] py-[5px] bg-[#323342] rounded-[5px]", 
                span { class: "text-sm font-semibold", "# Change Stance" }
            }
        }
    }
}