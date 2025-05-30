#![allow(non_snake_case)]
use bdk::prelude::*;
use dto::by_components::icons::arrows::ArrowRight;

use crate::{components::indicators::Indicator, route::Route};

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

                h1 { class: "w-full col-span-1 text-[32px] font-bold text-white max-tablet:!text-2xl pr-30 max-tablet:!col-span-full",
                    {title}
                }

                if with_line {
                    div { class: "col-span-1 w-full h-full flex flex-col items-center justify-center max-tablet:!hidden gap-5",
                        div { class: "w-full flex flex-col gap-5 items-end justify-end",
                            Link {
                                class: "btn group !gap-0",
                                to: Route::QuizzesPage {},
                                "Go to Quiz"
                                ArrowRight {
                                    width: "20",
                                    height: "20",
                                    class: "[&>path]:stroke-c-wg-50 group-hover:[&>path]:stroke-primary",
                                }
                            }
                            Link {
                                class: "btn group !gap-0",
                                to: Route::AdvocacyCampaignsByIdPage {
                                    id: 1,
                                },
                                "Go to Advocacy Campaign"
                                ArrowRight {
                                    width: "20",
                                    height: "20",
                                    class: "[&>path]:stroke-c-wg-50 group-hover:[&>path]:stroke-primary",
                                }

                            }
                        }
                        div { class: "w-full h-1 bg-c-wg-70" }
                    }
                }
            }

            div { class: "w-full flex flex-row gap-24",
                p { class: "w-full font-normal text-[15px]/22 text-c-wg-30 whitespace-pre-line max-[900px]:text-[15px]",
                    {description}
                }
                {children}
            }
        }
    }
}
