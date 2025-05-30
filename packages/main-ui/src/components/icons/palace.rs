use bdk::prelude::*;

#[component]
pub fn Palace(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "20".to_string())] width: String,
    #[props(default = "20".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            class,
            fill,
            height,
            view_box: "0 0 20 21",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            g { clip_path: "url(#clip0_1429_7550)",
                path {
                    d: "M20 0.308105H0V20.3081H20V0.308105Z",
                    fill: "white",
                    fill_opacity: "0.01",
                }
                path {
                    d: "M3.33301 7.6416H16.6663L9.99967 3.6416L3.33301 7.6416Z",
                    stroke: "#737373",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                }
                path {
                    d: "M16.6663 16.4751H3.33301",
                    stroke: "#737373",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                }
                path {
                    d: "M5 7.80811V16.1414",
                    stroke: "#737373",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                }
                path {
                    d: "M8.33301 7.80811V16.1414",
                    stroke: "#737373",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                }
                path {
                    d: "M11.667 7.80811V16.1414",
                    stroke: "#737373",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                }
                path {
                    d: "M15 7.80811V16.1414",
                    stroke: "#737373",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                }
            }
            defs {
                clipPath { id: "clip0_1429_7550",
                    rect {
                        fill: "white",
                        height: "20",
                        transform: "translate(0 0.308105)",
                        width: "20",
                    }
                }
            }
        }
    }
}
