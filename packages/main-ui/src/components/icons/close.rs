#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Close(#[props(default = "white".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M16.9498 7.05029L7.05029 16.9498M7.05029 7.05029L16.9498 16.9498",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
