#![allow(non_snake_case)]
use crate::components::icons::Logo;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Header(lang: Language, selected: i32) -> Element {
    let tr: HeaderTranslate = translate(&lang);

    rsx! {
        div { class: "fixed top-0 left-0 backdrop-blur-[20px] w-screen h-80 overflow-hidden flex items-center justify-center z-100",
            div { class: "w-full flex flex-row items-center justify-between gap-59 max-w-[1176px] mx-10",
                a { href: "#top", Logo {} }

                nav { class: "grow flex flex-row gap-[10px] text-secondary font-bold text-[15px]",
                    a {
                        class: "p-10 hover:text-white",
                        href: "#about",
                        color: if selected == 1 { "var(--color-primary)" },
                        {tr.menu_about}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "#politician-stance",
                        color: if selected == 2 { "var(--color-primary)" },
                        {tr.menu_stance}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "#community",
                        color: if selected == 3 { "var(--color-primary)" },
                        {tr.menu_community}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "#support",
                        color: if selected == 4 { "var(--color-primary)" },
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
