#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn ChatBubble() -> Element {
    rsx! {
        svg {
            width: "14",
            height: "14",
            view_box: "0 0 14 14",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M13 6.66669C13.0023 7.5466 12.7967 8.41461 12.4 9.20002C11.4406 11.1196 9.47927 12.3325 7.33333 12.3334C6.45342 12.3356 5.58541 12.1301 4.8 11.7334L1 13L2.26667 9.20002C1.86995 8.41461 1.66437 7.5466 1.66667 6.66669C1.6675 4.52075 2.88045 2.55938 4.8 1.60002C5.58541 1.20331 6.45342 0.997725 7.33333 1.00002H7.66667C10.5439 1.15875 12.8413 3.45615 13 6.33335V6.66669Z",
                stroke: "#74789E",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
