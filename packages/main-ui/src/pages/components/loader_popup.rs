#![allow(non_snake_case)]
use super::{
    login_failure_popup::LoginFailurePopup, signin_popup_footer::SigninPopupFooter,
    user_setup_popup::UserSetupPopup, wallet_signin_popup::WalletSigninPopup,
};
use crate::services::user_service::{UserEvent, UserService};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

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
                    popup
                        .open(rsx! {
                            UserSetupPopup {
                                class: "w-[390px]",
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
                    tracing::info!("User confirmed");
                    popup
                        .open(rsx! {
                            WalletSigninPopup {
                                class: "w-[400px] mx-[5px]",
                                logo,
                                logo_origin,
                                lang,
                            }
                        })
                        .with_id("wallet_signin_popup");
                }
                _ => {
                    tracing::error!("Failed to signup with Phantom");
                    popup
                        .open(rsx! {
                            LoginFailurePopup {
                                class: "w-[400px] mx-[5px]",
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
            div { class: "justify-start text-white font-bold text-xl/24", "{title}" }
            div { class: "w-full flex  justify-center items-center mt-[35px]",
                // TODO: border-t rounded
                div { class: "border-6 border-t-6 w-[82px] h-[82px] border-primary border-t-background rounded-full animate-spin" }
                div { class: "absolute w-[64px] h-[64px] bg-white rounded-full justify-center items-center flex",
                    div { class: "flex justify-center items-center", {display_logo} }
                }
            }
            div { class: "justify-center text-center text-white font-bold text-[16px] leading-[24px] mt-[35px]",
                "{load_message}"
            }
            SigninPopupFooter { lang }
        }
    }
}
