#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Clock() -> Element {
    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 20 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M15 5L16 4",
                stroke: "#8588AB",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 2H12",
                stroke: "#8588AB",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "10",
                cy: "11",
                r: "7",
                stroke: "#8588AB",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 11V8",
                stroke: "#8588AB",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
