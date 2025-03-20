#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::components::indicators::Indicator;

#[component]
pub fn SectionHeader(
    section_name: String,
    title: String,
    description: String,
    #[props(default = true)] with_line: bool,
) -> Element {
    rsx! {
        div { class: "hidden md:!block",
            DesktopSectionHeader {
                section_name: &section_name,
                title: &title,
                description: &description,
            }
        }
        div { class: "block md:!hidden",
            MobileSectionHeader {
                section_name: &section_name,
                title: &title,
                description: &description,
            }
        }
    }
}

#[component]
pub fn DesktopSectionHeader(
    section_name: String,
    title: String,
    description: String,
    #[props(default = true)] with_line: bool,
    children: Element,
) -> Element {
    let cols = if with_line {
        "grid-cols-2"
    } else {
        "grid-cols-1"
    };

    rsx! {
        div { class: "w-full flex flex-col justify-start items-start gap-20",
            Indicator { {section_name} }
            div { class: "w-full grid {cols} gap-24",
                h1 { class: "w-full col-span-1 text-[32px] font-bold text-white", {title} }
                if with_line {
                    div { class: "col-span-1 w-full h-full flex flex-col items-center justify-center",
                        div { class: "w-full h-1 bg-c-wg-70" }
                    }
                }
            }
            div { class: "w-full flex flex-row gap-24",
                p { class: "w-full font-normal text-[15px]/22 text-c-wg-30 whitespace-pre-line",
                    {description}
                }
                div { class: "w-full", {children} }
            }
        }
    }
}

#[component]
pub fn MobileSectionHeader(
    section_name: String,
    title: String,
    description: String,
    #[props(default = true)] with_line: bool,
    children: Element,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-col justify-start items-start gap-[10px]",

            Indicator {
                size: "w-[12px] h-[12px]".to_string(),
                gap: "gap-[10px]".to_string(),
                {section_name}
            }
            div { class: "w-full grid gap-24",
                h1 { class: "w-full col-span-1 text-[24px] font-bold text-white", {title} }
            }
            div { class: "w-full flex flex-row gap-24",
                p { class: "w-full font-normal text-[15px] text-c-wg-30 whitespace-pre-line",
                    {description}
                }
            }
        }
    }
}
