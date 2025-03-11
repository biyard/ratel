#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Wallet() -> Element {
    rsx! {
        svg {
            width: "25",
            height: "24",
            view_box: "0 0 25 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M19.2891 8V6C19.2891 4.89543 18.3936 4 17.2891 4H6.28906C5.18449 4 4.28906 4.89543 4.28906 6V18C4.28906 19.1046 5.18449 20 6.28906 20H17.2891C18.3936 20 19.2891 19.1046 19.2891 18V16",
                stroke: "white",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "13.2891",
                y: "8",
                width: "8",
                height: "8",
                rx: "1",
                stroke: "white",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "17.2891",
                cy: "12",
                r: "1.5",
                fill: "white",
            }
        }
    }
}
