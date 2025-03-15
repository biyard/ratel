#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::pages::components::{InputWithButton, Socials};

#[component]
pub fn Subscription(lang: Language) -> Element {
    let tr: SubscriptionTranslate = translate(&lang);

    rsx! {
        div { class: "w-full flex flex-col",
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
            div { class: "w-full flex flex-col items-center justify-center py-80 bg-footer gap-80",
                div { class: "w-full flex max-w-604 flex-col items-center gap-80",

                    div { class: "w-full flex flex-col gap-50 items-center",
                        div { class: "w-full flex flex-col items-start gap-5",
                            p { class: "text-c-cg-30 font-bold text-base/28", {tr.email} }
                            InputWithButton {
                                placeholder: tr.email_placeholder,
                                btn_name: tr.btn_subscribe,
                                r#type: "email",
                                onsubmit: |email| {
                                    btracing::info!("Subscribed by {}", email);
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

// #[derive(Debug, Clone, Copy, DioxusController)]
// pub struct Controller {
//     #[allow(dead_code)]
//     lang: Language,
// }

// impl Controller {
//     pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
//         let ctrl = Self { lang };

//         Ok(ctrl)
//     }
// }

translate! {
    SubscriptionTranslate;

    email: {
        ko: "이메일",
        en: "Email",
    },

    email_placeholder: {
        ko: "🖂 이메일을 입력하세요",
        en: "🖂 Input your mail",
    },

    btn_subscribe: {
        ko: "구독하기",
        en: "Subscribe",
    },
}
