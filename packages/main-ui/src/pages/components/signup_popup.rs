#![allow(non_snake_case)]
use super::{
    loader_popup::LoaderPopup, signin_popup_footer::SigninPopupFooter, wallet_popup::WalletPopup,
};
use crate::{components::icons, services::user_service::UserService};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn SignupPopup(
    #[props(default ="signup_popup".to_string())] id: String,
    lang: Language,
) -> Element {
    let tr = translate::<SignupPopupTranslate>(&lang);
    let mut user_service: UserService = use_context();
    let mut popup: PopupService = use_context();
    rsx! {
        div { id, class: "w-full max-w-400 mx-5 max-mobile:!max-w-full",
            div { class: "w-400 max-mobile:!w-full",
                div { class: "justify-start text-white font-bold text-xl/24", "{tr.title}" }
                div { class: "flex flex-col gap-10 mt-35",
                    div {
                        class: "w-full flex flex-row pl-20 py-22 bg-black border-[1px] rounded-[10px] justify-start items-center gap-17 cursor-pointer border-black hover:border-white",
                        onclick: move |_| async move {
                            tracing::debug!("Signup with Google clicked");
                            user_service.set_signer_type("google");
                            popup.open(rsx! {
                                LoaderPopup {
                                    lang,
                                    title: tr.loader_title,
                                    description: tr.loader_message,
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

                // div {
                //     class: "w-full flex flex-row pl-20 py-22 bg-black border-[1px] rounded-[10px] justify-start items-center gap-17 cursor-pointer border-black hover:border-white",
                //     onclick: move |_| {
                //         tracing::debug!("signup with wallet clicked");
                //         popup.open(rsx! {
                //             WalletPopup { lang }
                //         });
                //     },
                //     icons::Wallet {}
                //     div { class: "flex flex-col gap-3",
                //         span { class: "text-white text-base/19 font-semibold", "{tr.connect_wallet}" }
                //     }
                // }
                }
                SigninPopupFooter { lang }
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

    loader_title: {
        ko: "로그인",
        en: "Log in",
    }

    loader_message: {
        ko: "팝업에서 계정에 로그인하세요",
        en: "Sign into your account in the pop-up",
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
}
