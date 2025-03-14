#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn BottomSheet(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    onclick: EventHandler<()>,
) -> Element {
    let mut hover = use_signal(|| false);
    let fill = use_memo(move || if hover() { "#FCB300" } else { "none" });
    let stroke = use_memo(move || if hover() { "#000203" } else { "#777677" });

    rsx! {
        div { class: "w-full fixed bottom-0 left-0 flex flex-row items-center justify-center py-30 cursor-pointer z-11",
            svg {
                onmouseenter: move |_| hover.set(true),
                onmouseleave: move |_| hover.set(false),
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
        div { id: "bottom-sheet", class: "w-full fixed bottom-0 left-0 z-10",
            svg {
                fill: "none",
                view_box: "0 0 1921 146",
                width: "100%",
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
