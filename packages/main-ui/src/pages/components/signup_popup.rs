#![allow(non_snake_case)]
use super::{loader_popup::LoaderPopup, wallet_popup::WalletPopup};
use crate::{components::icons, services::user_service::UserService};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn SignupPopup(
    #[props(default ="signup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
) -> Element {
    let tr = translate::<SignupPopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();
    rsx! {
        div { id, class,
            div { class: "justify-start text-white font-bold text-xl/24", "{tr.title}" }
            div { class: "flex flex-col gap-10 mt-35",
                div {
                    class: "w-full flex flex-row pl-20 py-22 bg-black border-[1px] rounded-[10px] justify-start items-center gap-[17px] cursor-pointer hover:border-white",
                    onclick: move |_| async move {
                        tracing::debug!("Signup with Google clicked");
                        user_service.set_signer_type("google");
                        popup.open(rsx! {
                            LoaderPopup {
                                class: "w-[400px] mx-auto",
                                lang,
                                logo: rsx! {
                                    icons::Google { width: "50", height: "50" }
                                },
                                logo_origin: rsx! {
                                    icons::Google {}
                                },
                                msg: tr.continue_with_google,
                            }
                        });
                    },
                    icons::Google {}
                    div { class: "flex flex-col gap-3",
                        span { class: "text-white text-base/19 font-semibold",
                            "{tr.continue_with_google}"
                        }
                    }
                }

                div {
                    class: "w-full flex flex-row pl-20 py-22 bg-black border-[1px] rounded-[10px] justify-start items-center gap-[17px] cursor-pointer hover:border-white",
                    onclick: move |_| {
                        tracing::debug!("signup with wallet clicked");
                        popup.open(rsx! {
                            WalletPopup { class: "w-[400px] mx-[5px]", lang }
                        }).with_id("wallet_popup");
                    },
                    icons::Wallet {}
                    div { class: "flex flex-col gap-3",
                        span { class: "text-white text-base/19 font-semibold", "{tr.connect_wallet}" }
                    }
                }
            }
            // TODO: applying policy and terms.
            div { class: "flex flex-row gap-10 mt-35 justify-center",
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("Privacy policy clicked");
                    },
                    span { class: "text-neutral-400 text-xs/14 font-medium", "{tr.privacy_policy}" }
                }
                button {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        tracing::debug!("Privacy policy clicked");
                    },
                    span { class: "text-neutral-400 text-xs/14 font-medium", "{tr.term_of_service}" }
                }
            }
        }
    }
}

translate! {
    SignupPopupTranslate;

    title: {
        ko: "라텔에 참여하기",
        en: "Join the Movement",
    }

    continue_with_google: {
        ko: "Google로 계속하기",
        en: "Continue with Google",
    },

    continue_with_phantom_wallet: {
        ko: "팬텀 지갑으로 계속하기",
        en: "Continue with Phantom Wallet",
    },

    connect_wallet: {
        ko: "지갑 연결하기",
        en: "Connect Wallet",
    },

    need_wallet: {
        ko: "지갑 설치가 필요합니다",
        en: "Need Wallet",
    },

    privacy_policy: {
        ko: "개인정보 처리방침",
        en: "Privacy Policy",
    },

    term_of_service: {
        ko: "이용약관",
        en: "Term of Service",
    },
}
