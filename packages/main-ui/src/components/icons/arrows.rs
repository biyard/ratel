#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn RightArrow() -> Element {
    rsx! {
        svg {
            width: "16",
            height: "16",
            view_box: "0 0 16 16",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M6 12L10 8L6 4",
                stroke: "#828FA5",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
