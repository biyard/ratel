#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn MouseScrollDown(
    #[props(default = "#777677".to_string())] color: String,
    #[props(default = "none".to_string())] fill: String,
    onclick: EventHandler<()>,
    onmouseenter: EventHandler<()>,
    onmouseleave: EventHandler<()>,
) -> Element {
    rsx! {
        svg {
            onmouseenter: move |_| onmouseenter(()),
            onmouseleave: move |_| onmouseleave(()),
            onclick: move |_| onclick(()),
            width: "41",
            height: "40",
            view_box: "0 0 41 40",
            fill,
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M29.2696 11.139C29.2696 6.55308 25.5554 2.83545 20.9738 2.83545C16.3921 2.83545 12.678 6.55308 12.678 11.139V20.2462C12.678 24.8321 16.3921 28.5497 20.9738 28.5497C25.5554 28.5497 29.2696 24.8321 29.2696 20.2462V11.139Z",
                stroke: "{color}",
                stroke_width: "2",
            }
            path {
                d: "M22.0444 8.32109C22.0444 7.65835 21.5652 7.12109 20.974 7.12109C20.3828 7.12109 19.9036 7.65835 19.9036 8.32109V11.9211C19.9036 12.5838 20.3828 13.1211 20.974 13.1211C21.5652 13.1211 22.0444 12.5838 22.0444 11.9211V8.32109Z",
                fill: "{color}",
            }
            path {
                d: "M23.8071 34.5497L21.6809 36.6759C21.2904 37.0665 20.6572 37.0665 20.2667 36.6759L18.1405 34.5497",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
