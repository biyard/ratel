#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::logo::LogoWrapper;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "w-full items-start",
            div { class: "flex flex-col justify-start items-start pb-[50px] pt-[50px] text-[15px] font-semibold align-middle gap-3",
                // 로고
                LogoWrapper {}
                // 텍스트
                div {
                    class: "text-base flex flex-col",
                    style: "color: #8588AB;",
                    "Mine the Future, Cast Your"
                    br {}
                    "Predictions."
                }
            }
            hr { class: "w-full", style: "border-color: #424563;" }

            div { class: "w-full flex justify-between align-middle text-[15] font-bold py-[45px]",

                div { style: "color: #8588AB;", "© 2025  Biyard Corp. All rights reserved." }

                div { class: " flex gap-5", style: "color: #8588AB;",
                    div { "Privacy Policy" }
                    div { "Terms of Service" }
                }
            }
        }
    }
}
