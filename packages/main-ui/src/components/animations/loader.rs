#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::components::animations::lottie_animation::LottieAnimation;

#[component]
pub fn LoaderAnimation(
    #[props(default = "lottie-animation".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = true)] autoplay: bool,
    #[props(default = true)] loop_animation: bool,
    #[props(default = "100px".to_string())] width: String,
    #[props(default = "100px".to_string())] height: String,
) -> Element {
    const ASSET: Asset = asset!("/public/animations/loading.json");
    let src = ASSET.resolve().to_str().unwrap_or_default().to_string();
    rsx! {
        LottieAnimation {
            id,
            class,
            autoplay,
            loop_animation,
            width,
            height,
            src,
        }
    }
}
