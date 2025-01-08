#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn DoubleArrowDown(#[props(default = "white".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "20",
            height: "16",
            view_box: "0 0 20 16",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g {
                clip_path: "url(#clip0_346_4208)",
                path {
                    d: "M4.16797 4.875L10.0013 8.625L15.8346 4.875",
                    stroke: "{color}",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M4.16797 7.375L10.0013 11.125L15.8346 7.375",
                    stroke: "{color}",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}