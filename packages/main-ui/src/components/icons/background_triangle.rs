#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn BackgroundTriangle(#[props(default = "#1e1e1e".to_string())] color: String) -> Element {
    rsx! {
        svg {
            class: "w-full h-auto",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 100 25",
            preserve_aspect_ratio: "none",
            fill: "none",

            path {
                d: "M-0.5 18.7L100 0V37H-0.5V18.7Z",
                fill: "url(#paint0_linear_663_15455)",
            }

            defs {
                linearGradient {
                    id: "paint0_linear_663_15455",
                    x1: "50",
                    y1: "7.7",
                    x2: "50",
                    y2: "37",
                    gradient_units: "userSpaceOnUse",

                    stop { offset: "0", stop_color: "#1A1A1A" }
                    stop { offset: "0.65", stop_color: "#1E1E1E" }
                }
            }
        }
    }
}
