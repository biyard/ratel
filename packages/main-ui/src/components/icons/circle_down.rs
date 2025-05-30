#![allow(non_snake_case)]
use bdk::prelude::*;

pub fn CircleDown(
    fill: String,
    stroke: String,
    onclick: EventHandler<()>,
    onmouseenter: EventHandler<()>,
    onmouseleave: EventHandler<()>,
) -> Element {
    rsx! {
        svg {
            onmouseenter: move |_| onmouseenter(()),
            onmouseleave: move |_| onmouseleave(()),
            onclick: move |_| onclick(()),

            fill: "none",
            height: "41",
            view_box: "0 0 41 41",
            width: "41",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                cx: "20.2363",
                cy: "20.5",
                r: "19.5",
                stroke: "{stroke}",
                fill: "{fill}",
            }
            path {
                d: "M26.0693 18.9999L20.9431 24.1261C20.5526 24.5167 19.9194 24.5167 19.5289 24.1261L14.4027 18.9999",
                stroke: "{stroke}",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}
