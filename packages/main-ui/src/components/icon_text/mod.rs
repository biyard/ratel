#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn IconText(
    children: Element,
    text: String,
    background: Option<String>,
    #[props(default = "px-[14px] py-[8px]".to_string())] class: String,
) -> Element {
    rsx! {
        div {
            class: "flex flex-row items-center justify-center gap-[4px] {class} rounded-[8px]",
            style: match background {
                Some(bg) => format!("background: {}", bg),
                None => "".to_string(),
            },
            {children}
            span { class: "text-[14px] font-bold", "{text}" }
        }
    }
}
