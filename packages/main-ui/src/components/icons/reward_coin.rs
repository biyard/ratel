use bdk::prelude::*;

#[component]
pub fn RewardCoin(
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
            circle {
                cx: "10",
                cy: "10.3081",
                r: "7.5",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M12.9163 13.2248C12.9163 10.7248 10.8532 10.3081 10.0127 10.3081C8.14492 10.3081 7.08301 10.3081 7.08301 10.3081V13.2248",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M7.08301 7.39136H11.458C12.2634 7.39136 12.9163 8.04429 12.9163 8.84972V8.84972C12.9163 9.65515 12.2634 10.3081 11.458 10.3081H7.08301V7.39136Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}
