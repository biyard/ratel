#![allow(non_snake_case)]
use crate::{
    components::{confirm_popup::ConfirmPopup, socials::Socials},
    config,
    pages::components::InputWithButton,
};
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dto::Error;
use regex::Regex;

fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    email_regex.is_match(email)
}

#[component]
pub fn Subscription(lang: Language) -> Element {
    let tr: SubscriptionTranslate = translate(&lang);
    let mut popup: PopupService = use_context();

    rsx! {
        div { class: "w-full flex flex-col max-tablet:mt-40",
            div { class: "block max-tablet:!hidden",
                div { class: "w-full",
                    svg {
                        fill: "none",
                        view_box: "0 0 1921 146",
                        width: "100%",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M0.25 73.7684L1920.25 0V146H0.25V73.7684Z",
                            fill: "#191919",
                        }
                    }
                }
            }
            div { class: "w-full flex flex-col items-center justify-center py-80 bg-footer gap-80 max-tablet:!py-40",
                div { class: "w-full flex max-w-604 flex-col items-center gap-80 px-30",
                    //TODO(web): Make button separately and fix padding pixel
                    div { class: "w-full flex flex-col gap-50 items-center",
                        div { class: "w-full flex flex-col items-start gap-5",
                            p { class: "text-c-cg-30 font-bold text-base/28", {tr.email} }
                            InputWithButton {
                                placeholder: tr.email_placeholder,
                                btn_name: tr.btn_subscribe,
                                r#type: "email",
                                onsubmit: move |email: String| async move {
                                    if email.is_empty() {
                                        btracing::e!(lang, Error::EmptyInputValue);
                                        return;
                                    }
                                    if !is_valid_email(&email) {
                                        btracing::e!(lang, Error::InvalidInputValue);
                                        return;
                                    }
                                    let endpoint = config::get().main_api_endpoint;
                                    match dto::Subscription::get_client(endpoint).subscribe(email).await {
                                        Ok(_) => {
                                            let tr: SubscribedPopupTranslate = translate(&lang);
                                            popup.open(rsx! {
                                                ConfirmPopup {
                                                    lang,
                                                    title: tr.title,
                                                    description: tr.description,
                                                    btn_label: tr.btn_label,
                                                }
                                            });
                                        }
                                        Err(e) => {
                                            btracing::e!(lang, e);
                                        }
                                    }
                                },
                            }
                        }

                        Socials { class: "flex flex-row gap-50" }
                    }
                }
            }
        }
    }
}

translate! {
    SubscribedPopupTranslate;

    title: {
        ko: "구독 완료",
        en: "Subscription Confirmed",
    },

    description: {
        ko: "Ratel 구독을 환영합니다. 구독이 성공적으로 확인되었으며 이제 업데이트를 받게 됩니다.",
        en: "Thank you for subscribing to Ratel. Your subscription has been successfully confirmed, and you will now receive updates.",
    }

    btn_label: {
        ko: "확인",
        en: "Confirm",
    },
}

translate! {
    SubscriptionTranslate;

    email: {
        ko: "이메일",
        en: "Email",
    },

    email_placeholder: {
        // FIXME: change emoji cause U+1F582 is not supported in most fonts
        // https://www.fileformat.info/info/unicode/char/1f582/fontsupport.htm
        // ko: "🖂 이메일을 입력하세요",
        // en: "🖂 Please enter your email address",
        ko: "이메일을 입력하세요",
        en: "Please enter your email address",
    },

    btn_subscribe: {
        ko: "구독하기",
        en: "Subscribe",
    },
}
