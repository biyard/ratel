#![allow(non_snake_case)]
use crate::components::icons::MouseScrollDown;
use bdk::prelude::*;
#[component]
pub fn BottomSheet(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    onclick: EventHandler<()>,
) -> Element {
    let mut hover: Signal<bool> = use_signal(|| false);
    let fill = use_memo(move || if hover() { "#FCB300" } else { "none" });
    let stroke = use_memo(move || if hover() { "#000203" } else { "#777677" });

    rsx! {
        div { class: "w-full fixed bottom-0 left-0 flex flex-row items-center justify-center py-30 cursor-pointer z-11",
            MouseScrollDown {
                color: stroke,
                fill,
                onmouseenter: move |_| hover.set(true),
                onmouseleave: move |_| hover.set(false),
                onclick: move |_| onclick(()),
            }
        }
        div { id: "bottom-sheet", class: "w-full fixed bottom-0 left-0 z-10",
            svg {
                fill: "none",
                view_box: "0 0 1921 146",
                width: "100%",
                height: "100",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M0.25 73.7684L1920.25 0V146H0.25V73.7684Z",
                    fill: "url(#paint0_linear_218_870)",
                }
                defs {
                    linearGradient {
                        gradient_units: "userSpaceOnUse",
                        id: "paint0_linear_218_870",
                        x1: "1124.25",
                        x2: "1124.25",
                        y1: "30.7368",
                        y2: "146",
                        stop { stop_color: "#1A1A1A" }
                        stop { offset: "0.65", stop_color: "#1E1E1E" }
                    }
                }
            }
        }
    }
}
