#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::meta::MetaPage;
use subscription::Subscription;

use super::components::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let tr: TopTranslate = translate(&lang);
    let image = asset!("/public/logos/logo.png");

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

    rsx! {
        MetaPage { title: "Ratel", description: tr.description, image: "{image}" }
        div { class: "w-full flex flex-col justify-start items-center",
            div { class: "w-full flex flex-col justify-start items-center max-desktop:px-30 max-tablet:gap-58",
                Top { lang }
                About { lang }
                PoliticianStance { lang }
                Community { lang }
                Support { lang }
            }
            Subscription { lang }
            Footer { lang }
        }
    }
}
