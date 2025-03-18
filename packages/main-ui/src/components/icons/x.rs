#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn X(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = 29)] size: i32,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "{size}",
            view_box: "0 0 29 29",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path {
                d: "M21.6305 2.2395H25.5662L16.9678 12.0669L27.0832 25.4398H19.1629L12.9595 17.3292L5.86144 25.4398H1.92334L11.1202 14.9283L1.4165 2.2395H9.5378L15.1451 9.65289L21.6305 2.2395ZM20.2492 23.0841H22.43L8.3528 4.47149H6.01254L20.2492 23.0841Z",
                fill: "white",
            }
        }
    }
}
