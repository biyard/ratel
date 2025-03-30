#![allow(non_snake_case)]
use super::SignupPopup;
use crate::{
    components::{icons::Logo, socials::Socials},
    route::Route,
    services::user_service::UserService,
};
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
        // div { class: "block max-tablet:!hidden",
        DesktopHeader { lang, selected }
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
    let user_service: UserService = use_context();
    let mut expanded = use_signal(|| false);
    let mobile_menu = use_memo(move || {
        let expanded = expanded();

        if expanded {
            "max-tablet:right-0"
        } else {
            "max-tablet:-right-full"
        }
    });

    rsx! {
        div { class: "fixed top-0 left-0 backdrop-blur-[20px] w-screen h-80 flex items-center justify-center z-100 max-tablet:!h-48",
            div { class: "w-full flex flex-row items-center justify-between gap-59 max-w-[1176px] mx-10",
                div { class: "flex flex-row items-center justify-between max-tablet:w-full",
                    a { href: "/#top", Logo {} }
                    button {
                        class: "hidden max-tablet:!block cursor-pointer",
                        visibility: if expanded() { "hidden" },
                        onclick: move |_| {
                            tracing::debug!("menu clicked");
                            expanded.set(true);
                        },
                        AlignJustify {
                            width: 32,
                            height: 32,
                            class: "[&>path]:stroke-white",
                        }
                    }

                    button {
                        class: "w-32 h-32 flex justify-center items-center rounded-[4px] border border-c-wg-70 cursor-pointer",
                        visibility: if expanded() { "visible" } else { "hidden" },
                        display: if !expanded() { "none" },
                        onclick: move |_| {
                            expanded.set(false);
                        },
                        ArrowRight {
                            width: 20,
                            height: 20,
                            class: "[&>path]:stroke-white",
                        }
                    }
                }

                div { class: "w-full flex flex-row gap-59 max-tablet:!fixed max-tablet:!w-screen max-tablet:!h-screen max-tablet:top-68 max-tablet:transition-all {mobile_menu} max-tablet:items-center max-tablet:justify-center max-tablet:z-999 max-tablet:bg-background max-tablet:!flex-col max-tablet:!gap-168",
                    nav { class: "grow flex flex-row gap-10 text-secondary font-bold text-[15px] max-tablet:!gap-20 max-tablet:!grow-0 max-tablet:!flex-col max-tablet:!items-center max-tablet:!justify-center max-tablet:z-100 max-tablet:!text-xl",
                        A {
                            lang,
                            selected: selected == 1,
                            href: "/#about",
                            onclick: move |_| expanded.set(false),
                            {tr.menu_about}
                        }
                        A {
                            lang,
                            selected: selected == 2,
                            href: "/#politician-stance",
                            onclick: move |_| expanded.set(false),
                            {tr.menu_stance}
                        }
                        A {
                            lang,
                            selected: selected == 3,
                            href: "/#community",
                            onclick: move |_| expanded.set(false),
                            {tr.menu_community}
                        }
                        A {
                            lang,
                            selected: selected == 4,
                            href: "/#support",
                            onclick: move |_| expanded.set(false),
                            {tr.menu_support}
                        }
                    }

                    div { class: "flex flex-row gap-10 max-tablet:!flex-col max-tablet:!items-center max-tablet:!justify-center",
                        Link {
                            class: "px-20 py-10 bg-primary hover:bg-hover text-black text-sm cursor-pointer rounded-full font-bold max-tablet:flex items-center justify-center hidden",
                            to: Route::PreparingPage { lang },
                            onclick: move |_| expanded.set(false),
                            {tr.get_ratel}
                        }

                        if let Some((_, _)) = user_service.get_user_info() {
                            button {
                                class: "text-neutral-500 text-[15px] font-bold p-10 hover:text-hover cursor-pointer",
                                onclick: move |_| async move {
                                    tracing::debug!("my reward clicked");
                                    expanded.set(false);
                                },
                                {tr.my_rewards}
                            }
                        } else {
                            button {
                                class: "p-10 text-[15px] font-bold text-secondary hover:text-hover cursor-pointer max-tablet:!px-44 max-tablet:!py-15",
                                onclick: move |_| {
                                    tracing::debug!("Sign in clicked");
                                    expanded.set(false);
                                    popup.open(rsx! {
                                        SignupPopup { class: "w-full max-w-400 mx-5", lang }
                                    }).with_id("signup_popup");
                                },
                                {tr.login}
                            }
                        }
                        Link {
                            class: "px-20 py-10 bg-primary hover:bg-hover text-black text-sm cursor-pointer rounded-full font-bold flex items-center justify-center max-tablet:hidden",
                            to: Route::PreparingPage { lang },
                            onclick: move |_| expanded.set(false),
                            {tr.get_ratel}
                        }

                        Socials {
                            class: "flex-row items-center justify-center gap-50 hidden mt-34 max-tablet:!flex",
                            size: 28,
                            onclick: move |_| expanded.set(false),
                        }
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
            div { class: "w-32 h-32 flex justify-center items-center rounded-[4px] p-10 border border-c-wg-70 py-58",
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
            div { class: "fixed top-0 w-full h-[48px] bg-background flex items-center z-[100]",
                div { class: "w-full px-[16px] py-[6px] flex flex-row justify-between items-center",
                    a {
                        href: "/#top",
                        class: "flex justify-start items-center",
                        style: "transform: scale(0.65); margin-left: -26px;",
                        Logo {}
                    }
                    button { class: "cursor-pointer", onclick: toggle_menu, {toggle_icon} }
                }
            }

            div {
                id: "menus",
                class: "fixed top-[48px] left-0 w-full h-full flex flex-col justify-center items-center bg-background z-[100]",
                style: match expanded() {
                    true => "display: block;",
                    false => "display: none;",
                },
                div { class: "h-full max-h-400 flex flex-col justify-center items-center gap-[20px]",

                    A {
                        href: "/#about",
                        selected: selected == 1,
                        lang,
                        onclick: move |_| expanded.set(false),
                        button { onclick: move |_| close_menu(),
                            p { class: "p-10 font-bold text-xl text-neutral-500 hover:text-primary",
                                {tr.menu_about}
                            }
                        }
                    }

                    A {
                        href: "/#politician-stance",
                        lang,
                        selected: selected == 2,
                        onclick: move |_| expanded.set(false),
                        button { onclick: move |_| close_menu(),
                            p { class: "p-10 font-bold text-xl text-neutral-500 hover:text-primary",
                                {tr.menu_stance}
                            }
                        }
                    }

                    A {
                        href: "/#community",
                        lang,
                        selected: selected == 3,
                        onclick: move |_| expanded.set(false),
                        button { onclick: move |_| close_menu(),
                            p { class: "p-10 font-bold text-xl text-neutral-500 hover:text-primary",
                                {tr.menu_community}
                            }
                        }
                    }

                    A {
                        href: "/#support",
                        lang,
                        selected: selected == 4,
                        onclick: move |_| expanded.set(false),
                        button { onclick: move |_| close_menu(),
                            p { class: "p-10 font-bold text-xl text-neutral-500 hover:text-primary",
                                {tr.menu_support}
                            }
                        }
                    }
                }

                div { class: "flex flex-col gap-48",
                    div { class: "flex flex-col items-center gap-[10px]",
                        //get ratel button
                        button {
                            div { class: "w-176 h-48 px-20 py-10 flex justify-center items-center bg-primary text-black text-[15px] rounded-[50px] font-bold",
                                {tr.get_ratel}
                            }
                        }
                        //sign in button
                        button {
                            class: "w-85 h-43 p-10 flex justify-center items-center text-neutral-500 text-xl font-bold",
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
pub fn A(
    children: Element,
    lang: Language,
    href: String,
    selected: bool,
    onclick: Option<EventHandler<()>>,
) -> Element {
    let current_path: Route = use_route();
    let is_home = matches!(current_path, Route::HomePage { .. });

    rsx! {
        if is_home {
            a {
                class: "p-10 hover:text-white",
                href,
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
                color: if selected { "var(--color-primary)" },
                {children}
            }
        } else {
            Link {
                class: "p-10 hover:text-white",
                to: Route::HomePage { lang },
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
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

    my_rewards: {
        ko: "내 리워드",
        en: "My Rewards",
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
