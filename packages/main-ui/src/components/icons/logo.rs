#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Logo(#[props(default = 22)] width: u32, #[props(default = 26)] height: u32) -> Element {
    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            view_box: "0 0 22 26",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { "clip-path": "url(#clip0_184_65)",
                path {
                    d: "M21.23 6.48L21.27 19.5L10.63 25.5L0 19.5L0.04 6.48L10.63 0.5L21.23 6.48Z",
                    fill: "white",
                }
                path {
                    d: "M3.99 16.92V8.69998L0.04 6.47998L0 19.5L10.63 25.5V20.72L3.99 16.92Z",
                    fill: "#D4D5D8",
                }
                path {
                    d: "M17.2799 16.92V8.69998L21.2299 6.47998L21.2699 19.5L10.6299 25.5V20.72L17.2799 16.92Z",
                    fill: "#C0B086",
                }
                path {
                    d: "M0.0400391 6.48L3.99004 8.7L10.62 4.95L17.28 8.7L21.23 6.48L10.63 0.5",
                    fill: "#F8F1C0",
                }
                path {
                    d: "M17.2702 16.99L10.6502 20.76L3.99023 16.99L10.6302 13.24L17.2702 16.99Z",
                    fill: "#F8F1C0",
                }
                path {
                    d: "M3.99023 8.70001L10.6202 4.95001V13.24L3.99023 16.99V8.70001Z",
                    fill: "#C0B086",
                }
                path {
                    d: "M17.2699 8.70001L10.6299 4.95001V13.24L17.2599 16.99L17.2699 8.70001Z",
                    fill: "#D4D5D8",
                }
            }
            defs {
                clipPath { id: "clip0_184_65",
                    rect {
                        width: "21.27",
                        height: "25",
                        fill: "white",
                        transform: "translate(0 0.5)",
                    }
                }
            }
        }
    }
}
