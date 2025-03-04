#![allow(non_snake_case)]
use crate::route::*;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

use super::{
    i18n::HeaderTranslate,
    menus::Menus,
    signup_popup::SignupPopup,
};

use crate::{
    components::{button::Button, logo::LogoWrapper},
    pages::i18n::PatronageButtonTranslate,
    services::user_service::UserService,
    theme::Theme,
};

#[component]
pub fn Header(lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-row items-center justify-between w-full pt-[47px] pb-[39px]",
            LogoWrapper {}
            Menus { class: "grow flex flex-row justify-end px-[30px]", lang }
            PatronageBox { lang }
            HeaderTails { lang }
        }
    }
}

#[component]
pub fn PatronageBox(lang: Language) -> Element {
    let tr: PatronageButtonTranslate = translate(&lang);
    let navigator = use_navigator();

    rsx! {
        div {
            button {
                class: "h-11 px-4 py-2.5 bg-[#414462] rounded-lg justify-center items-center gap-2.5 inline-flex",
                onclick: move |_| {
                    navigator.push(Route::NewPatronPage { lang });
                },
                div { class: "text-white text-base font-bold font-['Inter'] uppercase leading-snug",
                    "{tr.button}"
                }
            }
        }
    }
}

#[component]
pub fn HeaderTails(lang: Language) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let mut popup: PopupService = use_context();

    let mut user_service: UserService = use_context();
    tracing::debug!("lang: {:?}", lang);

    let i18n_header: HeaderTranslate = translate(&lang);
    tracing::debug!("i18n_header: {:?}", i18n_header);

    let onclick = move |_| {
        tracing::debug!("signup button clicked");
        popup
            .open(rsx! {
                SignupPopup { class: "w-[400px]", lang: lang.clone() }
            })
            .with_id("signup")
            .with_title(i18n_header.signup);
    };

    let logout = move |_| {
        tracing::debug!("logout button clicked");
        user_service.logout();
    };

    rsx! {
        div { class: "flex flex-row gap-[30px] justify-start items-center",
            if let Some((nickname, profile_url)) = user_service.get_user_info() {
                Button { color: "{theme.primary00}", onclick: logout, "{i18n_header.logout}" }
                div { class: "flex flex-row gap-[8px] items-center justify-center",
                    img {
                        class: "w-[24px] h-[24px] object-contain rounded-full",
                        src: "{profile_url}",
                    }
                    p { class: "{theme.font_theme.exbold15} uppercase", "{nickname}" }
                }
            } else {
                Button { color: "{theme.primary00}", onclick, "{i18n_header.login}" }
                Button {
                    color: "{theme.primary00}",
                    background: "{theme.primary06}",
                    onclick,
                    "{i18n_header.signup}"
                }
            }
        }
    }
}
