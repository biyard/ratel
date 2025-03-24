#![allow(non_snake_case)]
use super::{
    login_failure_popup::LoginFailurePopup, signin_popup_footer::SigninPopupFooter,
    signup_popup::SignupPopup, user_setup_popup::UserSetupPopup,
    wallet_signin_popup::WalletSigninPopup,
};
use crate::{
    components::icons,
    services::user_service::{UserEvent, UserService},
};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn LoaderPopup(
    #[props(default ="loader_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
    title: String,
    description: String,
    logo: Element,
    logo_origin: Element,
    msg: String,
) -> Element {
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();
    let display_logo = logo.clone();
    let load_message = description.clone();
    use_effect(move || {
        let logo_origin = logo_origin.clone();
        let logo = logo.clone();
        let msg = msg.clone();
        let description = description.clone();
        spawn(async move {
            match user_service.login().await {
                UserEvent::Signup(principal, email, nickname, profile_url) => {
                    tracing::debug!("Email: {}, Principal: {}", email, principal);
                    popup
                        .open(rsx! {
                            UserSetupPopup {
                                class: "w-390",
                                nickname,
                                profile_url,
                                email,
                                principal,
                                lang,
                            }
                        })
                        .with_id("user_setup_popup");
                }
                UserEvent::Login => {
                    popup.close();
                }
                UserEvent::Confirmed => {
                    tracing::debug!("User confirmed");
                    popup
                        .open(rsx! {
                            WalletSigninPopup {
                                class: "w-400 mx-5",
                                logo,
                                logo_origin,
                                lang,
                                msg,
                            }
                        })
                        .with_id("wallet_signin_popup");
                }
                _ => {
                    tracing::error!("Failed to signup with {:?}", msg);
                    popup
                        .open(rsx! {
                            LoginFailurePopup {
                                class: "w-400 mx-5",
                                description,
                                logo,
                                logo_origin,
                                msg,
                                lang,
                            }
                        })
                        .with_id("login_failure_popup");
                }
            }
        });
    });

    rsx! {
        div { id, class,
            div { class: "flex flex-row justify-start gap-12",
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("backward button clicked");
                        popup.open(rsx! {
                            SignupPopup { class: "w-[400px] mx-[5px]", lang }
                        }).with_id("signup_popup");
                    },
                    span { class: "text-neutral-400 text-xs/14 font-medium",
                        icons::LeftArrow { color: "white", width: "24", height: "24" }
                    }
                }
                div { class: "justify-start text-white font-bold text-xl/24", {title} }
            }
            div { class: "w-full flex  justify-center items-center mt-35",
                // TODO: border-t rounded
                div { class: "border-6 border-t-6 w-82 h-82 border-primary border-t-background rounded-full animate-spin" }
                div { class: "absolute w-64 h-64 bg-white rounded-full justify-center items-center flex",
                    div { class: "flex justify-center items-center", {display_logo} }
                }
            }
            div { class: "justify-center text-center text-white font-bold text-base/24 mt-35",
                "{load_message}"
            }
            SigninPopupFooter { lang }
        }
    }
}
