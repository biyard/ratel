#![allow(non_snake_case)]
use bdk::prelude::*;

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

pub fn DownArrow() -> Element {
    rsx! {
        svg {
            width: "16",
            height: "16",
            view_box: "0 0 16 16",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M4 6L8 10L12 6",
                stroke: "#828FA5",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn LeftArrow(
    #[props(default = "#404761".to_string())] color: String,
    #[props(default = "16".to_string())] width: String,
    #[props(default = "16".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            width,
            height,
            view_box: "0 0 16 16",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M10 4L6 8L10 12",
                stroke: color,
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

pub fn BigRightArrow() -> Element {
    rsx! {
        svg {
            width: "12",
            height: "24",
            view_box: "0 0 11 11",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                d: "M6 12L10 8L6 4",
                stroke: "#ffffff",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
