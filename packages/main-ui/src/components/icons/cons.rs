#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Cons(#[props(default = "#DA4447".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "16",
            height: "16",
            view_box: "0 0 16 16",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9.33247 10.0013V12.668C9.33247 13.7725 8.43704 14.668 7.33247 14.668L5.55469 9.55686V1.33464H12.1858C12.8507 1.32712 13.4194 1.81059 13.5191 2.46797L14.4391 8.46797C14.4978 8.85473 14.3837 9.24776 14.127 9.54296C13.8703 9.83816 13.497 10.0057 13.1058 10.0013H9.33247Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.88759 1.33464H2.66536C1.92898 1.33464 1.33203 1.93159 1.33203 2.66797V7.33464C1.33203 8.07102 1.92898 8.66797 2.66536 8.66797H2.88759V1.33464Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
        }
    }
}