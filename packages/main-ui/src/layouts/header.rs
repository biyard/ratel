#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;

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
    let mut popup: PopupService = use_context();

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
                onclick: move |_| {
                    tracing::debug!("회원가입 버튼 클릭");
                    popup.open(rsx! {
                        div {
                            onclick: move |e| {
                                e.stop_propagation();
                            },
                            div {
                                class: "flex flex-col gap-[20px]",
                                "테스트"
                            }
                        }
                    }).with_id("signup").with_title("회원가입");
                },
                "회원가입"
            }
        }
    }
}
