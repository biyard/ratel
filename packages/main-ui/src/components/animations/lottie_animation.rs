#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::utils::lottie::LottieController;

#[component]
pub fn LottieAnimation(
    id: String,
    class: String,
    autoplay: bool,
    loop_animation: bool,
    width: String,
    height: String,
    src: String,
) -> Element {
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None);
    let id_clone: String = id.clone();
    rsx! {
        div { class: "lottie-wrapper",
            div {
                id,
                class: "lottie-container {class} flex items-center justify-center",
                style: "width: {width}; height: {height};",
                onmounted: move |_| {
                    let id_clone = id_clone.clone();
                    let src = src.clone();
                    spawn(async move {
                        match LottieController::load(&id_clone, &src, loop_animation, autoplay).await
                        {
                            Ok(ctrl) => {
                                tracing::debug!("Animation loaded successfully");
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
            }
        }
    }
}
