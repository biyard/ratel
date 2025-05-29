use bdk::prelude::*;

use super::components::*;
use crate::{
    components::{loader::Loader, popup_zone::PopupZone, quick_menu::QuickMenu},
    route::Route,
    services::user_service::UserService,
};
use by_components::meta::MetaSeoTemplate;

#[component]
pub fn LandingLayout(#[props(default = Language::En)] lang: Language) -> Element {
    let user_service: UserService = use_context();
    let nav = use_navigator();
    let current_path: Route = use_route();
    let current_path2 = current_path.clone();

    use_effect(move || {
        if !matches!(current_path2, Route::QuizzesPage {})
            && !matches!(current_path2, Route::ResultsPage { .. })
            && crate::config::get().experiment
            && user_service.loggedin()
        {
            nav.push(Route::IndexPage {});
        }
    });

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
            } else if y <= height * 4.7 {
                4
            } else if y <= height * 5.7 {
                5
            } else {
                6
            };

            return i;
        }
        #[cfg(not(feature = "web"))]
        0
    });
    let is_home = matches!(current_path, Route::LandingPage { .. });

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

    let user_service: UserService = use_context();
    use_effect(move || {
        if user_service.loggedin() {
            // TODO:
        }
    });

    rsx! {
        PopupZone {}
        MetaSeoTemplate {
            lang,
            title: "Ratel",
            keywords: "ratel, crypto, policy, south korea, ecosystem, politicians, supportive policies, track, crypto stances, vote, legislation, propose, DAO-driven improvements, shape, thriving future, industry, democracy",
            author: "Ratel Foundation",
            url: "https://ratel.foundation",
        }

        QuickMenu {}
        div { class: "w-full h-full bg-background text-white",
            Header { lang, selected: selected() }
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "absolute bg-background w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                        Loader { class: "w-200" }
                    }
                },
                div { class: "w-full min-h-[100vh] overflow-x-hidden scroll-smooth flex flex-col items-center justify-center",
                    Outlet::<Route> {}
                }
            }
        }
        if selected() != 5 && is_home {
            div { class: "max-[900px]:!hidden",
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
}
