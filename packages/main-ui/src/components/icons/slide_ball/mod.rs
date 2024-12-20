#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn SlideBall() -> Element {
    rsx! {
        svg {
            width: "25",
            height: "24",
            view_box: "0 0 25 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            // Background Circle
            circle {
                cx: "12.3711",
                cy: "12",
                r: "12",
                fill: "#D9D9D9",
            }

            // Left Bar
            line {
                x1: "8.37061",
                y1: "9",
                x2: "8.37061",
                y2: "16",
                stroke: "#ADBCD7",
                stroke_width: "2",
                stroke_linecap: "round",
            }

            // Middle Bar
            line {
                x1: "12.3711",
                y1: "8",
                x2: "12.3711",
                y2: "17",
                stroke: "#ADBCD7",
                stroke_width: "2",
                stroke_linecap: "round",
            }

            // Right Bar
            line {
                x1: "16.3706",
                y1: "9",
                x2: "16.3706",
                y2: "16",
                stroke: "#ADBCD7",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
