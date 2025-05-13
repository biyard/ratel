#![allow(non_snake_case)]
use bdk::prelude::{by_components::icons::video_camera::VideoPlaylist, *};
use by_components::meta::MetaPage;
use subscription::Subscription;

use super::components::*;
use super::controller::*;

#[component]
pub fn HomePage(#[props(default = Language::En)] lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: TopTranslate = translate(&lang);
    let image = asset!("/public/logos/logo.png");
    let mut muted = use_signal(|| true);

    #[cfg(feature = "web")]
    {
        use_effect(|| {
            use wasm_bindgen::JsCast;
            use web_sys::{Element, window};

            if let Some(window) = window() {
                let location = window.location();
                if let Ok(hash) = location.hash() {
                    tracing::debug!("hash :{hash}");

                    if !hash.is_empty() {
                        let document = window.document().unwrap();
                        // web_sys::console::log_1(&hash);
                        if let Some(target) = document.query_selector(&hash).ok().flatten() {
                            let scroll = web_sys::ScrollIntoViewOptions::new();
                            scroll.set_behavior(web_sys::ScrollBehavior::Smooth);

                            let _ = target
                                .dyn_ref::<Element>()
                                .unwrap()
                                .scroll_into_view_with_scroll_into_view_options(&scroll);
                        }
                    }
                }
            }
        });
    }

    rsx! {
        MetaPage { title: "Ratel", description: tr.description, image: "{image}" }
        div { class: "absolute top-0 left-0 w-full h-auto",
            div { class: "absolute inset-0 bg-background/95 z-1" }
                div { class: "absolute relative w-full z-0",
            video {
                class: "w-full h-screen object-cover",
                autoplay: true,
                muted: muted(),
                playsinline: "false",
                onmouseenter: move |_| muted.set(false),
                onmouseleave: move |_| muted.set(true),
                r#loop: true,
                src: asset!("/public/videos/ratel.mp4"),
            }
        }
        }

        a {
            class: "absolute bottom-10 right-10 group z-20",
            href: "https://www.youtube.com/watch?v=v36xYZf70iM",
            target: "_blank",
            VideoPlaylist {
                class: "transition-all [&>path]:stroke-white [&>path]:stroke-1 hover:[&>path]:stroke-primary cursor-pointer",
                width: "40",
                height: "40",
            }
        }

        div { class: "w-full flex flex-col justify-start items-center z-2",
            div { class: "w-full flex flex-col justify-start items-center max-desktop:px-30 max-tablet:gap-58",
                Top { lang }
                About { lang }
                PresidentialElection {
                    lang,
                    candidates: ctrl.candidates().unwrap_or_default(),
                }
                PoliticianStance { lang }
                Community { lang }
                Support { lang }
            }
            Subscription { lang }
            Footer { lang }
        }
    }
}
