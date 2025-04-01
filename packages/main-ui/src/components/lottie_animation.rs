#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::utils::lottie::LottieController;

#[component]
pub fn LottieAnimation(
    #[props(default = "lottie-animation".to_string())] id: String,
    #[props(default = "".to_string())] class: String,
    src: String,
    #[props(default = true)] autoplay: bool,
    #[props(default = true)] loop_animation: bool,
    #[props(default = "100%".to_string())] width: String,
    #[props(default = "100%".to_string())] height: String,
) -> Element {
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None);
    let id_clone: String = id.clone();

    rsx! {
        div {
            class: "lottie-wrapper",
            onmounted: move |_| {
                let id_clone = id_clone.clone();
                let src = src.clone();
                tracing::debug!("LottieAnimation component mounted");
                spawn(async move {
                    match LottieController::load(&id_clone, &src, loop_animation, autoplay).await
                    {
                        Ok(ctrl) => {
                            tracing::debug!("Animation loaded successfully");
                            ctrl.play();
                            loading.set(false);
                        }
                        Err(e) => {
                            tracing::error!("Failed to load animation: {:?}", e);
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                });
            },
            div {
                id,
                class: "lottie-container {class} flex items-center justify-center",
                style: "width: {width}; height: {height};",

                if (loading)() {
                    div { class: "text-white", "Loading animation..." }
                } else if let Some(error_msg) = (error)() {
                    div { class: "text-red-500", "Error: {error_msg}" }
                } else {

                }
            }
        }
    }
}
