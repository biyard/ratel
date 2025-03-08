#![allow(non_snake_case)]
use crate::components::icons::Logo;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Header(lang: Language) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    #[cfg(feature = "web")]
    let mut scroll_position = use_signal(|| 0.0);
    let selected = use_memo(move || {
        #[cfg(feature = "web")]
        {
            let y = scroll_position();
            tracing::debug!("y = {y}");
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
            } else {
                4
            };

            return i;
        }
        #[cfg(not(feature = "web"))]
        0
    });

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
        div { class: "fixed top-0 left-0 w-screen h-80 overflow-hidden flex items-center justify-center z-100",
            div { class: "w-full flex flex-row items-center justify-between gap-59 max-w-[1176px] mx-10",
                a { href: "#top", Logo {} }

                nav { class: "grow flex flex-row gap-[10px] text-secondary font-bold text-[15px]",
                    a {
                        class: "p-10 hover:text-white",
                        href: "#about",
                        color: if selected() == 1 { "var(--color-primary)" },
                        {tr.menu_about}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "#politician-stance",
                        color: if selected() == 2 { "var(--color-primary)" },
                        {tr.menu_stance}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "#community",
                        color: if selected() == 3 { "var(--color-primary)" },
                        {tr.menu_community}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "#support",
                        color: if selected() == 4 { "var(--color-primary)" },
                        {tr.menu_support}
                    }
                }

                div { class: "flex flex-row gap-10",
                    button { class: "p-10 text-[15px] font-bold text-secondary hover:text-hover cursor-pointer",
                        {tr.login}
                    }
                    button { class: "px-20 py-10 bg-primary hover:bg-hover text-black text-[14px] cursor-pointer rounded-full font-bold",
                        {tr.get_ratel}
                    }
                }

            }
        }
    }
}

translate! {
    HeaderTranslate;

    menu_about: {
        ko: "About",
        en: "About",
    }

    menu_stance: {
        ko: "Politician stance",
        en: "Politician stance",
    }

    menu_community: {
        ko: "Community",
        en: "Community",
    }

    menu_support: {
        ko: "Support",
        en: "Support",
    }

    reward: {
        ko: "나의 보상",
        en: "My Rewards",
    }

    login: {
        ko: "로그인",
        en: "Sign in",
    }

    get_ratel: {
        ko: "$RATEL 받기",
        en: "GET $RATEL",
    }
}
