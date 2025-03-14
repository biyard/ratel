#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn MissionCard(
    no: String,
    title: String,
    description: String,

    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {..attributes,
            div { class: "px-30 py-40 bg-[#1A1A1A] w-full h-full flex flex-col items-start justify-between text-[#DEE1E6] rounded-[15px]",
                div { class: "flex flex-col gap-20 text-xl/22 font-bold",
                    p { {no} }
                    h2 { {title} }
                }
                p { class: "text-[15px]/22", {description} }

            }
        }
    }
}
