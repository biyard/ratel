#![allow(non_snake_case)]
use super::InputWithButton;
use super::Socials;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Footer(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

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
            footer { class: "w-full flex flex-col items-center justify-center h-367 pt-80 pb-24 bg-footer gap-80",
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

                    div { class: "flex flex-row gap-10 items-center text-copyright font-normal text-sm/22",
                        span { {tr.copyright} }
                        a { class: "hover:text-white", href: "/privacy", {tr.privacy} }
                        a { class: "hover:text-white", href: "/terms", {tr.terms} }
                        a { class: "hover:text-white", href: "/sitemap", {tr.sitemap} }
                    }
                }
            }
        }
    }
}

translate! {
    FooterTranslate;
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

    copyright: {
        ko: "© 2025 Ratel Foundation.",
        en: "© 2025 Ratel Foundation.",
    }

    privacy: {
        ko: "• 개인 정보 보호 정책",
        en: "• Privacy",
    },

    terms: {
        ko: "• 서비스 약관",
        en: "• Terms",
    },

    sitemap: {
        ko: "• 사이트맵",
        en: "• Sitemap",
    },
}
