#![allow(non_snake_case)]
use super::SignupPopup;
use crate::{components::icons::Logo, route::Route, services::user_service::UserService};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn Header(lang: Language, selected: i32) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let current_path: Route = use_route();
    let selected_menu = use_memo(move || match current_path {
        Route::PoliticiansPage { .. } => 2,
        Route::PoliticiansByIdPage { .. } => 2,
        _ => 0,
    });
    let mut user_service: UserService = use_context();

    rsx! {
        div { class: "fixed top-0 left-0 backdrop-blur-[20px] w-screen h-80 overflow-hidden flex items-center justify-center z-100",
            div { class: "w-full flex flex-row items-center justify-between gap-59 max-w-[1176px] mx-10",
                a { href: "/#top", Logo {} }

                nav { class: "grow flex flex-row gap-10 text-secondary font-bold text-[15px]",
                    A { lang, selected: selected == 1, href: "/#about", {tr.menu_about} }
                    A {
                        lang,
                        selected: selected == 2 || selected_menu == 2,
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
                    if let Some((_, _)) = user_service.get_user_info() {
                        button {
                            class: "text-neutral-500 text-[15px] font-bold p-10 hover:text-hover cursor-pointer",
                            onclick: move |_| async move {
                                tracing::debug!("logout button clicked");
                                user_service.logout().await;
                            },
                            {tr.my_rewards}
                        }
                    } else {
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

    my_rewards: {
        ko: "내 리워드",
        en: "My Rewards",
    }

    get_ratel: {
        ko: "$RATEL 받기",
        en: "GET $RATEL",
    }
}
