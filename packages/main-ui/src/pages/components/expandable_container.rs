#![allow(non_snake_case, dead_code, unused_variables)]
use dioxus::prelude::*;

#[component]
pub fn ExpandableContainer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    tag: String,
    total_count: i64,
    icon: Element,
    expanded: bool,
    children: Element,
) -> Element {
    let rotate = if expanded { "rotate-0" } else { "rotate-270" };
    let grow = if expanded { "grow w-full" } else { "" };
    let direction = if expanded { "flex-row" } else { "flex-col" };

    rsx! {
        div {..attributes,
            div { class: "transition-all duration-300 ease-in-out flex flex-col items-center justify-start gap-20 w-full bg-bg rounded-[20px] cursor-pointer px-30 py-40",
                div { class: "flex {direction} items-center justify-start gap-10",
                    span { class: "font-bold text-[32px]/22", "{total_count}" }
                    div { class: "{grow} text-white text-xl/22 font-bold flex flex-row items-center justify-start",
                        {tag}
                    }
                    div { class: "rotate-270", {icon} }
                }

                div { class: "overflow-hidden", width: if expanded { "100%" } else { "0" }, {children} }
            }

        }
    }
}
