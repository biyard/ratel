#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Sort(
    #[props(default = "white".to_string())] color: String,
    #[props(default = false)] filled: bool,
) -> Element {
    rsx! {
        svg {
            width: "12",
            height: "13",
            view_box: "0 0 12 13",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: if filled {
                    "M1.5 2H10.5L7 6.9085V11H5V6.9085L1.5 2Z"
                } else {
                    "M1.28516 2H10.7152L7.25016 6.9085V11H4.75016V6.9085L1.28516 2ZM3.21516 3L5.75016 6.5915V10H6.25016V6.5915L8.78516 3H3.21516Z"
                },
                fill: "{color}",
            }
        }
    }
}