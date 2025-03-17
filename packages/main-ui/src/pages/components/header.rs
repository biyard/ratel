#![allow(non_snake_case)]
use super::SignupPopup;
use crate::{components::icons::Logo, pages::components::Socials, route::Route};
use bdk::prelude::{
    by_components::{
        icons::{alignments::AlignJustify, arrows::ArrowRight},
        responsive::{Responsive, ResponsiveService},
    },
    *,
};
use dioxus_popup::PopupService;

#[component]
pub fn ResponsiveHeader(
    #[props(default ="".to_string())] class: String,
    lang: Language,
    selected: i32,
) -> Element {
    let responsive_service: ResponsiveService = use_context();

    rsx! {
        Responsive {
            if responsive_service.width() > 1200.0 {
                Header { class, lang, selected }
            } else {
                MobileHeader { class, lang, selected }
            }
        }
    }
}
// Header { lang: lang.clone(), selected }

#[component]
pub fn Header(
    #[props(default ="".to_string())] class: String,
    lang: Language,
    selected: i32,
) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let current_path: Route = use_route();
    let selected_menu = use_memo(move || match current_path {
        Route::PoliticiansPage { .. } => 2,
        _ => 0,
    });

    rsx! {
        div { class: "fixed top-0 left-0 backdrop-blur-[20px] w-screen h-80 overflow-hidden flex items-center justify-center z-100",
            div { class: "w-full flex flex-row items-center justify-between gap-59 max-w-[1176px] mx-10",
                a { href: "/#top", Logo {} }

                nav { class: "grow flex flex-row gap-[10px] text-secondary font-bold text-[15px]",
                    a {
                        class: "p-10 hover:text-white",
                        href: "/#about",
                        color: if selected == 1 { "var(--color-primary)" },
                        {tr.menu_about}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "/#politician-stance",
                        color: if selected == 2 || selected_menu == 2 { "var(--color-primary)" },
                        {tr.menu_stance}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "/#community",
                        color: if selected == 3 { "var(--color-primary)" },
                        {tr.menu_community}
                    }
                    a {
                        class: "p-10 hover:text-white",
                        href: "/#support",
                        color: if selected == 4 { "var(--color-primary)" },
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
                    button { class: "px-20 py-10 bg-primary hover:bg-hover text-black text-[14px] cursor-pointer rounded-full font-bold",
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
    // let mut popup: PopupService = use_context();
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
        _ => 0,
    });

    let toggle_icon = if expanded() {
        rsx! {
            div { class: "w-[32px] h-[32px] flex justify-center items-center rounded-[4px] p-[10px] border-[1px] !border-[#464646]",
                div {
                    ArrowRight {
                        width: 20,
                        height: 20,
                        class: "[&>path]:stroke-[#ffffff]",
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                AlignJustify {
                    width: 32,
                    height: 32,
                    class: "[&>path]:stroke-[#ffffff]",
                }
            }
        }
    };

    //structure
    // 'menu / logo / login button' or 'menu / logo'

    //mene structure
    // 'About / Politician stance / Community / Support / Sign in / Get $Ratel'

    rsx! {
        div { class: "fixed top-0 left-0 w-full h-full grow flex flex-col justify-start items-center z-[100]",
            div { class: "w-full h-[48px] bg-[#1E1E1E] flex items-center",
                a {
                    href: "/#top",
                    class: "w-full px-[16px] py-[6px] flex flex-row justify-between items-center",
                    div {
                        class: "flex justify-start items-center",
                        style: "transform: scale(0.65); margin-left: -26px;",
                        Logo {}
                    }
                    button { onclick: toggle_menu, {toggle_icon} }
                }
            }
            if expanded() {
                div {
                    id: "menus",
                    class: "w-full h-full flex flex-col justify-center items-center bg-[#1E1E1E]",
                    div { class: "mb-[140px] flex flex-col justify-center items-center gap-[20px]",
                        a { href: "/#about", onclick: move |_| close_menu(),
                            p {
                                class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                color: if selected == 1 { "var(--color-primary)" },
                                {tr.menu_about}
                            }
                        }
                        a {
                            href: "/#politician-stance",
                            onclick: move |_| close_menu(),
                            p {
                                class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                color: if selected == 2 { "var(--color-primary)" },
                                {tr.menu_stance}
                            }
                        }
                        a {
                            href: "/#community",
                            onclick: move |_| close_menu(),
                            p {
                                class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                color: if selected == 3 { "var(--color-primary)" },
                                {tr.menu_community}
                            }
                        }
                        a { href: "/#support", onclick: move |_| close_menu(),
                            p {
                                class: "p-[10px] font-bold text-[20px] text-[#737373] hover:text-[#FCB300]",
                                color: if selected == 4 { "var(--color-primary)" },
                                {tr.menu_support}
                            }
                        }
                    }

                    div { class: "flex flex-col gap-[48px]",
                        //get ratel button
                        button {
                            div { class: "w-[176px] h-[48px] px-[20px] py-[10px] flex justify-center items-center bg-[#FCB300] text-black text-[15px] rounded-[50px] font-bold",
                                {tr.get_ratel}
                            }
                        }

                        //social icon
                        div { class: "w-full h-full flex flex-row justify-center items-center",
                            Socials { class: "flex flex-row gap-[40px] [&>a>svg]:w-19 [&>a>svg]:h-19" }
                        }
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
