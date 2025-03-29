#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::components::indicators::Indicator;

#[component]
pub fn SectionHeader(
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

                h1 { class: "w-full col-span-1 text-[32px] font-bold text-white max-[900px]:!text-[24px] max-[900px]:!w-screen pr-[30px]",
                    {title}
                }


                div { class: "block max-[900px]:!hidden",
                    if with_line {
                        div { class: "col-span-1 w-full h-full flex flex-col items-center justify-center",
                            div { class: "w-full h-1 bg-c-wg-70" }
                        }
                    }
                }
            }

            div { class: "w-full flex flex-row gap-24",
                p { class: "w-full font-normal text-[15px]/22 text-c-wg-30 whitespace-pre-line max-[900px]:text-[15px]",
                    {description}
                }
                div { class: "w-full max-[900px]:hidden", {children} }
            }
        }
    }
}
