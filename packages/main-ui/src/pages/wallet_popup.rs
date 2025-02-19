#![allow(non_snake_case)]
use super::{i18n::WalletPopupTranslate, user_setup_popup::UserSetupPopup};
use crate::{
    components::icons,
    services::user_service::{UserEvent, UserService},
};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

#[component]
pub fn WalletPopup(
    #[props(default ="wallet_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
) -> Element {
    let tr = translate::<WalletPopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();

    rsx! {

        div {
            id,
            class: "w-full flex flex-row my-[10px] p-[8px] bg-[#AB9FF1] rounded-[8px] justify-start items-center gap-[17px] cursor-pointer hover:bg-[#9A8EFF]",
            onclick: move |_| async move {
                tracing::debug!("Signup with Phantom clicked");
                match user_service.phantom_login().await {
                    UserEvent::Signup(principal, email, nickname, profile_url) => {
                        popup.open(rsx! {
                            UserSetupPopup {
                                class: "w-[400px]",
                                nickname,
                                profile_url,
                                email,
                                principal,
                                lang: lang.clone(),
                            }
                        });
                    }
                    UserEvent::Login => {
                        popup.close();
                    }
                    _ => {
                        tracing::error!("Failed to signup with Phantom");
                        popup.close();
                    }
                };
            },

            div { class: "rounded-[8px] w-[62px] h-[62px] flex items-center justify-center",
                icons::Phantom {}
            }
            div { class: "flex flex-col gap-[3px]",
                span { class: "text-white text-[16px] leading-[16px] font-extrabold",
                    "{tr.title}"
                }
                span { class: "text-white text-[14px] leading-[13px] fond-regular", "{tr.sub_text}" }
            }
        }
    }
}
