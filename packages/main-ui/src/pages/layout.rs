#![allow(non_snake_case)]
use bdk::prelude::{
    by_components::responsive::{Responsive, ResponsiveService},
    *,
};
use dioxus_popup::PopupZone;

use super::components::*;
use crate::route::Route;
use by_components::{loaders::cube_loader::CubeLoader, meta::MetaSeoTemplate};

#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        RootBase { lang,
            Footer { lang }
        }
    }
}

#[component]
pub fn RootBase(lang: Language, children: Element) -> Element {
    #[cfg(feature = "web")]
    let mut scroll_position = use_signal(|| 0.0);
    let selected = use_memo(move || {
        #[cfg(feature = "web")]
        {
            let y = scroll_position();
            let height = match web_sys::window() {
                Some(window) => window
                    .inner_height()
                    .unwrap_or_default()
                    .as_f64()
                    .unwrap_or_default(),
                None => 0.0,
            };

            #[cfg(not(feature = "web"))]
            let height = 0.0;

            let i = if y < height * 0.7 {
                0
            } else if y < height * 1.7 {
                1
            } else if y < height * 2.7 {
                2
            } else if y < height * 3.7 {
                3
            } else if y <= height * 4.0 {
                4
            } else {
                5
            };

            return i;
        }
        #[cfg(not(feature = "web"))]
        0
    });
    let current_path: Route = use_route();
    let is_home = matches!(current_path, Route::HomePage { .. });
    let responsive_service: ResponsiveService = use_context();

    #[cfg(feature = "web")]
    let _ = use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let script = r#"
            window.addEventListener('scroll', () => {
                dioxus.send(`${window.scrollY}`);
            });
        "#;
        let mut eval = document::eval(script);

        loop {
            let y = eval
                .recv::<String>()
                .await
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or_default();
            scroll_position.set(y);
        }
    });

    rsx! {
        PopupZone {
            background_color: "rgba(26, 26, 26, 1)",
            border_class: "shadow-[#FFCE4740] shadow-2xl", // FIXME: need shadow size to 100px
        }
        MetaSeoTemplate {
            lang,
            title: "Ratel",
            keywords: "ratel, crypto, policy, south korea, ecosystem, politicians, supportive policies, track, crypto stances, vote, legislation, propose, DAO-driven improvements, shape, thriving future, industry, democracy",
            author: "Ratel Foundation",
            url: "https://ratel.foundation",
        }
        div { class: "w-full h-full bg-background text-white",
            ResponsiveHeader { lang, selected: selected() }
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                        CubeLoader {}
                    }
                },
                Responsive {
                    if responsive_service.width() > 1200.0 {
                        div { class: "w-full overflow-x-hidden scroll-smooth flex flex-col items-center justify-center mt-80",
                            Outlet::<Route> {}
                            Footer { lang }
                        }
                    } else {
                        div { class: "w-full overflow-x-hidden scroll-smooth flex flex-col items-center justify-center mt-[130px]",
                            Outlet::<Route> {}
                            Footer { lang }
                        }
                    }
                }
            }
        }
        if selected() != 5 && is_home {
            BottomSheet {
                onclick: move |_| {
                    let height = match web_sys::window() {
                        Some(window) => {
                            window.inner_height().unwrap_or_default().as_f64().unwrap_or_default()
                        }
                        None => 0.0,
                    };
                    let next_position = height * (selected() + 1) as f64;
                    let script = format!(
                        "window.scrollTo({{ top: {next_position}, behavior: 'smooth' }});",
                    );
                    let _ = document::eval(&script);
                },
            }
        }
    }
}
