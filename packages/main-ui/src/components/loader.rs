#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn Loader(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        div {..attributes,
            lottie-player {
                src: asset!("/public/animations/loading.json"),
                class: "w-full",
                "autoplay": true,
                "loop": true,
            }
        }
    }
}
