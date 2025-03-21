#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn BackgroundTriangle(#[props(default = "#1e1e1e".to_string())] color: String) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "393",
            height: "146",
            view_box: "0 0 393 146",
            fill: "none",

            path {
                d: "M-0.5 73.7684L392.5 0V146H-0.5V73.7684Z",
                fill: "url(#paint0_linear_663_15455)",
            }

            defs {
                linearGradient {
                    id: "paint0_linear_663_15455",
                    x1: "230.169",
                    y1: "30.7368",
                    x2: "230.169",
                    y2: "146",
                    gradient_units: "userSpaceOnUse",

                    stop { offset: "0", stop_color: "#1A1A1A" }
                    stop { offset: "0.65", stop_color: "#1E1E1E" }
                }
            }
        }
    }
}
