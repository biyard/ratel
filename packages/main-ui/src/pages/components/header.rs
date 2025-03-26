#![allow(non_snake_case)]
use super::SignupPopup;
use crate::{components::icons::Logo, pages::components::Socials, route::Route};
use bdk::prelude::{
    by_components::icons::{alignments::AlignJustify, arrows::ArrowRight},
    *,
};
use dioxus_popup::PopupService;

#[component]
pub fn Header(
    #[props(default ="".to_string())] class: String,
    lang: Language,
    selected: i32,
) -> Element {
    rsx! {
        div { class: "block max-[900px]:!hidden",
            DesktopHeader { lang, selected }
        }
        div { class: "hidden max-[900px]:!block",
            MobileHeader { lang, selected }
        }
    }
}

#[component]
pub fn DesktopHeader(lang: Language, selected: i32) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let current_path: Route = use_route();
    let selected_menu = use_memo(move || match current_path {
        Route::PoliticiansPage { .. } => 2,
        Route::PoliticiansByIdPage { .. } => 2,
        _ => 0,
    });

    rsx! {
        div { class: "fixed top-0 left-0 backdrop-blur-[20px] w-screen h-80 overflow-hidden flex items-center justify-center z-100",
            div { class: "w-full flex flex-row items-center justify-between gap-59 max-w-[1176px] mx-10",
                a { href: "/#top", Logo {} }

                nav { class: "grow flex flex-row gap-10 text-secondary font-bold text-[15px]",
                    A { lang, selected: selected == 1, href: "/#about", {tr.menu_about} }
                    A {
                        lang,
                        selected: selected == 2,
                        href: "/#politician-stance",
                        {tr.menu_stance}
                    }
                    A {
                        lang,
                        selected: selected == 3,
                        href: "/#community",
                        {tr.menu_community}
                    }
                    A {
                        lang,
                        selected: selected == 4,
                        href: "/#support",
                        {tr.menu_support}
                    }
                }

                div { class: "flex flex-row gap-10",
                    button {
                        class: "p-10 text-[15px] font-bold text-secondary hover:text-hover cursor-pointer",
                        onclick: move |_| {
                            tracing::debug!("Sign in clicked");
                            popup.open(rsx! {
                                SignupPopup { class: "w-[400px] mx-[5px]", lang }
                            }).with_id("signup_popup");
                        },
                        {tr.login}
                    }
                    button { class: "px-20 py-10 bg-primary hover:bg-hover text-black text-sm cursor-pointer rounded-full font-bold",
                        {tr.get_ratel}
                    }
                }
            }
        }
    }
}

#[component]
pub fn MobileHeader(
    #[props(default ="".to_string())] class: String,
    lang: Language,
    selected: i32,
) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let current_path: Route = use_route();
    let mut expanded = use_signal(|| false);
    let toggle_menu = move |_| {
        expanded.set(!expanded());
    };
    let mut close_menu = move || {
        expanded.set(false);
    };
    let selected_menu = use_memo(move || match current_path {
        Route::PoliticiansPage { .. } => 2,
        Route::PoliticiansByIdPage { .. } => 2,
        _ => 0,
    });

    let toggle_icon = if expanded() {
        rsx! {
            div { class: "w-32 h-32 flex justify-center items-center rounded-[4px] p-10 border border-[#464646]",
                div {
                    ArrowRight {
                        width: 20,
                        height: 20,
                        class: "[&>path]:stroke-white",
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                AlignJustify { width: 32, height: 32, class: "[&>path]:stroke-white" }
            }
        }
    };

    rsx! {
        div { class: "w-full h-full grow flex flex-col justify-start items-center",
            div { class: "fixed top-0 w-full h-[48px] bg-[#1E1E1E] flex items-center z-[100]",
                div { class: "w-full px-[16px] py-[6px] flex flex-row justify-between items-center",
                    a {
                        href: "/#top",
                        class: "flex justify-start items-center",
                        style: "transform: scale(0.65); margin-left: -26px;",
                        Logo {}
                    }
                    button { onclick: toggle_menu, {toggle_icon} }
                }
            }

            div {
                id: "menus",
                class: "fixed top-[48px] left-0 w-full h-full flex flex-col justify-center items-center bg-[#1E1E1E] z-[100]",
                style: match expanded() {
                    true => "display: block;",
                    false => "display: none;",
                },
                div { class: "h-full max-h-400 flex flex-col justify-center items-center gap-[20px]",

                    A { href: "/#about", selected: selected == 1, lang,
                        button { onclick: move |_| close_menu(),
                            p { class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                {tr.menu_about}
                            }
                        }
                    }

                    A {
                        href: "/#politician-stance",
                        lang,
                        selected: selected == 2,
                        button { onclick: move |_| close_menu(),
                            p { class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                {tr.menu_stance}
                            }
                        }
                    }

                    A {
                        href: "/#community",
                        lang,
                        selected: selected == 3,
                        button { onclick: move |_| close_menu(),
                            p { class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                {tr.menu_community}
                            }
                        }
                    }

                    A {
                        href: "/#support",
                        lang,
                        selected: selected == 4,
                        button { onclick: move |_| close_menu(),
                            p { class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                {tr.menu_support}
                            }
                        }
                    }
                }

                div { class: "flex flex-col gap-[48px]",
                    div { class: "flex flex-col items-center gap-[10px]",
                        //get ratel button
                        button {
                            div { class: "w-[176px] h-[48px] px-[20px] py-[10px] flex justify-center items-center bg-[#FCB300] text-black text-[15px] rounded-[50px] font-bold",
                                {tr.get_ratel}
                            }
                        }
                        //sign in button
                        button {
                            class: "w-[85px] h-[43px] p-[10px] flex justify-center items-center text-[#737373] text-[20px] font-bold",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    SignupPopup { class: "w-[320px] mx-[5px]", lang }
                                }).with_id("signup_popup");
                            },
                            {tr.sign_in}
                        }
                    }
                    Socials {
                        class: "flex flex-row items-center justify-center gap-50",
                        size: 28,
                    }
                }
            }
        }
    }
}

#[component]
pub fn A(children: Element, lang: Language, href: String, selected: bool) -> Element {
    let current_path: Route = use_route();
    let is_home = matches!(current_path, Route::HomePage { .. });

    rsx! {
        if is_home {
            a {
                class: "p-10 hover:text-white",
                href,
                color: if selected { "var(--color-primary)" },
                {children}
            }
        } else {
            Link {
                class: "p-10 hover:text-white",
                to: Route::HomePage { lang },
                color: if selected { "var(--color-primary)" },
                {children}
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

    sign_in: {
        ko: "로그인하기",
        en: "Sign in",
    }
}
