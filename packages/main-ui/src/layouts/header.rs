#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;

use crate::{
    components::{button::Button, logo::LogoWrapper},
    layouts::{signup_popup::SignupPopup, user_setup_popup::UserSetupPopup},
    services::user_service::{UserEvent, UserService},
    theme::Theme,
};

#[component]
pub fn Header() -> Element {
    rsx! {
        div { class: "flex flex-row items-center justify-between w-full pt-[47px] pb-[39px]",
            LogoWrapper {}
            HeaderTails {}
        }
    }
}

#[component]
pub fn HeaderTails() -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let mut popup: PopupService = use_context();

    let mut user_service: UserService = use_context();

    let onclick = move |_| {
        tracing::debug!("회원가입 버튼 클릭");
        popup
            .open(rsx! {
                SignupPopup {
                    class: "w-[400px]",
                    onclick: move |_| async move {
                        tracing::debug!("Google로 계속하기 버튼 클릭");
                        match user_service.login().await {
                            UserEvent::Signup(principal, email, nickname, profile_url) => {
                                popup.open(rsx! {
                                    UserSetupPopup {
                                        class: "w-[400px]",
                                        nickname,
                                        profile_url,
                                        email,
                                        principal,
                                    }
                                });
                            }
                            UserEvent::Login => {
                                popup.close();
                            }
                            _ => {
                                tracing::error!("회원가입 실패");
                                popup.close();
                            }
                        };
                    },
                }
            })
            .with_id("signup")
            .with_title("회원가입");
    };

    rsx! {
        div { class: "flex flex-row gap-[30px] justify-start items-center",
            if let Some((nickname, profile_url)) = user_service.get_user_info() {
                div { class: "flex flex-row gap-[8px] items-center justify-center",
                    img {
                        class: "w-[24px] h-[24px] object-contain rounded-full",
                        src: "{profile_url}",
                    }
                    p { class: "{theme.font_theme.exbold15} uppercase", "{nickname}" }
                }
            } else {
                Button { color: "{theme.primary00}", onclick, "로그인" }
                Button {
                    color: "{theme.primary00}",
                    background: "{theme.primary06}",
                    onclick,
                    "회원가입"
                }
            }
        }
    }
}
