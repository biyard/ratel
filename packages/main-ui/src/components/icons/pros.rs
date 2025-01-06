#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Pros(#[props(default = "#68D36C".to_string())] color: String) -> Element {
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
                d: "M9.33247 5.9987V3.33203C9.33247 2.22746 8.43704 1.33203 7.33247 1.33203L5.55469 6.44314V14.6654H12.1858C12.8507 14.6729 13.4194 14.1894 13.5191 13.532L14.4391 7.53203C14.4978 7.14527 14.3837 6.75224 14.127 6.45704C13.8703 6.16184 13.497 5.99427 13.1058 5.9987H9.33247Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.88759 14.6654H2.66536C1.92898 14.6654 1.33203 14.0684 1.33203 13.332V8.66536C1.33203 7.92898 1.92898 7.33203 2.66536 7.33203H2.88759V14.6654Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
        }
    }
}