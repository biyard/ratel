#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn Indicator(
    #[props(default = "w-12 h-12".to_string())] size: String,
    #[props(default = "gap-5".to_string())] gap: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-row items-center {gap} text-primary font-bold text-sm",
            div { class: "{size} bg-primary rounded-full" }
            {children}
        }
    }
}
