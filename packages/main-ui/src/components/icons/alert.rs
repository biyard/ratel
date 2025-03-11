#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn AlertCircle(#[props(default = "white".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "18",
            height: "19",
            view_box: "0 0 18 19",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M9 5.61111V10.2778",
                stroke: "{color}",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 3.61111V8.27778M15 7.5C15 11.366 11.866 14.5 8 14.5C4.13401 14.5 1 11.366 1 7.5C1 3.63401 4.13401 0.5 8 0.5C11.866 0.5 15 3.63401 15 7.5Z",
                stroke: "{color}",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "9.00043",
                cy: "13.0004",
                r: "0.777778",
                fill: "{color}",
            }
        }
    }
}
