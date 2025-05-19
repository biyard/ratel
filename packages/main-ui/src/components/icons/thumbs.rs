#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn ThumbsUp(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = "white".to_string())] color: String,
    #[props(default = 31)] size: i32,
) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 31 31",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M17.8675 11.3516V6.35156C17.8675 4.28049 16.1886 2.60156 14.1175 2.60156L10.7842 12.1849V27.6016H23.2175C24.4641 27.6157 25.5306 26.7092 25.7175 25.4766L27.4425 14.2266C27.5526 13.5014 27.3386 12.7644 26.8573 12.211C26.376 11.6575 25.676 11.3433 24.9425 11.3516H17.8675Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.78385 27.6016H5.36719C3.98648 27.6016 2.86719 26.4823 2.86719 25.1016V16.3516C2.86719 14.9709 3.98648 13.8516 5.36719 13.8516H5.78385V27.6016Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
        }
    }
}

#[component]
pub fn ThumbsDown(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = "white".to_string())] color: String,
    #[props(default = 31)] size: i32,
) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 31 31",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M17.5589 19.4043V24.4043C17.5589 26.4754 15.88 28.1543 13.8089 28.1543L10.4756 18.571V3.1543H22.9089C24.1555 3.14021 25.222 4.0467 25.4089 5.2793L27.1339 16.5293C27.244 17.2545 27.03 17.9914 26.5487 18.5449C26.0674 19.0984 25.3674 19.4126 24.6339 19.4043H17.5589Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.47526 3.1543H5.05859C3.67788 3.1543 2.55859 4.27359 2.55859 5.6543V14.4043C2.55859 15.785 3.67788 16.9043 5.05859 16.9043H5.47526V3.1543Z",
                fill: "{color}",
                stroke: "{color}",
                stroke_width: "1.5",
            }
        }
    }
}
