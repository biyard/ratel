#![allow(non_snake_case)]
use super::SignupPopup;
use crate::components::{
    button::{primary_button::PrimaryA, secondary_botton::SecondaryButton},
    icons::CharacterSymbol,
    socials::Socials,
};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn Top(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let tr: TopTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let white_paper = asset!("/public/documents/Ratel-Token-White-Paper.pdf");

    rsx! {
        div {
            id: "top",
            class: "w-full h-screen flex flex-col items-center justify-center gap-100 max-tablet:!h-auto max-tablet:!gap-123 max-tablet:!pt-123 max-tablet:!pb-58",
            ..attributes,
            div { class: "flex flex-col items-center justify-center gap-32",
                div { class: "max-[900px]:!scale-70", CharacterSymbol {} }
                h1 { class: "text-5xl/56 text-center font-bold text-white whitespace-pre-line max-[900px]:text-[28px]",
                    {tr.slogan}
                }
                p { class: "text-lg text-center font-normal text-c-wg-30 whitespace-pre-line max-[900px]: text-[15px]",
                    {tr.description}
                }

                Socials { class: "flex flex-row gap-50" }
            }

            div { class: "w-full flex flex-row gap-10 max-[900px]:!flex-col",
                PrimaryA { href: "/public/documents/Ratel-Token-White-Paper.pdf", {tr.btn_learn} }

                SecondaryButton {
                    onclick: move |_| {
                        tracing::debug!("Learn more clicked");
                        popup.open(rsx! {
                            SignupPopup { class: "w-400", lang }
                        });
                    },
                    {tr.btn_join}
                }
            }
        }
    }
}

translate! {
    TopTranslate;

    slogan: {
        ko: "공정한 암호화폐 법안을 위해\n함께 싸워요!",
        en: "Join the Fight\nfor Fair Crypto Legislation",
    },
    description: {
        ko: "한국 시민과 의원을 연결하는 첫 번째 플랫폼으로\n암호화폐 산업을 위한 제도 개혁을 추진합니다. 함께 하실래요?",
        en: "The first platform connecting South Korea’s citizens with lawmakers to drive\ninstitutional reform for the crypto industry. Are you with us?",
    },

    mobile_description: {
        ko: "한국 시민과 의원을 연결하는 첫 번째 플랫폼으로 암호화폐 산업을 위한 제도 개혁을 추진합니다. 함께 하실래요?",
        en: "The first platform connecting South Korea’s citizens with lawmakers to drive institutional reform for the crypto industry. Are you with us?",
    },

    btn_learn: {
        ko: "$RATEL에 대해 더 알아보기",
        en: "LEARN MORE ABOUT $RATEL",
    },
    btn_join: {
        ko: "지금 참여하기",
        en: "JOIN THE MOVEMENT",
    },
}
