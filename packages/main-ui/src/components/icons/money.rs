#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Money() -> Element {
    rsx! {
        svg {
            width: "20",
            height: "20",
            view_box: "0 0 20 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",

            // Circle
            circle {
                cx: "10",
                cy: "10",
                r: "7.5",
                stroke: "#C5BB7B",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

            // First path
            path {
                d: "M6.6665 7.5V13.3333L9.99984 8.14815L13.3332 13.3333V7.5",
                stroke: "#C5BB7B",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }

            // Second path (Left fill rectangle)
            path {
                d: "M5 10.0834C4.58579 10.0834 4.25 10.4192 4.25 10.8334C4.25 11.2476 4.58579 11.5834 5 11.5834V10.0834ZM7.5 11.5834H8.25V10.0834H7.5V11.5834ZM5 11.5834H7.5V10.0834H5V11.5834Z",
                fill: "#C5BB7B",
            }

            // Third path (Right fill rectangle)
            path {
                d: "M12.5 11.5834L11.75 11.5834L11.75 10.0834L12.5 10.0834L12.5 11.5834ZM15 10.0834C15.4142 10.0834 15.75 10.4192 15.75 10.8334C15.75 11.2476 15.4142 11.5834 15 11.5834L15 10.0834ZM12.5 10.0834L15 10.0834L15 11.5834L12.5 11.5834L12.5 10.0834Z",
                fill: "#C5BB7B",
            }
        }
    }
}
