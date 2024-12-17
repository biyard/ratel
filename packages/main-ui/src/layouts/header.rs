#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::{button::Button, logo::LogoWrapper},
    theme::Theme,
};

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "flex flex-row items-center justify-between w-full pt-[47px] pb-[39px]",
            LogoWrapper { }
            HeaderTails { }
        }
    }
}

#[component]
pub fn HeaderTails() -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();

    rsx! {
        div {
            class: "flex flex-row gap-[30px] justify-start items-center",
            Button {
                color: "{theme.primary00}",
                onclick: move |_| {},
                "로그인"
            }
            Button {
                color: "{theme.primary00}",
                background: "{theme.primary06}",
                onclick: move |_| {},
                "회원가입"
            }
        }
    }
}
