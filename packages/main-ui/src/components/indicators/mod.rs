#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn Indicator(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row items-center gap-5 text-primary font-bold text-sm",
            div { class: "w-12 h-12 bg-primary rounded-full" }
            {children}
        }
    }
}
