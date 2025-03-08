#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

use crate::components::icons::{Bsky, CharacterSymbol, Telegram, X};

#[component]
pub fn Top(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let tr: TopTranslate = translate(&lang);

    rsx! {
        div {
            id: "top",
            class: "w-screen h-screen flex flex-col items-center justify-center gap-100",
            ..attributes,
            div { class: "flex flex-col items-center justify-center gap-32",
                CharacterSymbol {}
                h1 { class: "text-[48px] text-center font-bold leading-[56px] text-white whitespace-pre-line",
                    {tr.slogan}
                }
                p { class: "text-[18px] text-center text-white font-normal text-[#AEAEAE] whitespace-pre-line",
                    {tr.description}
                }
                div { class: "flex flex-row gap-50",
                    X {}
                    Bsky {}
                    Telegram {}
                }
            }

            div { class: "flex flex-row gap-20",
                // TODO: implement downloading whitepaper
                button { class: "bg-primary hover:bg-hover px-40 py-20 gap-10 flex items-center justify-center text-black font-bold text-base rounded-[10px]",
                    {tr.btn_learn}
                }

                // TODO: implement Sign in
                button {
                    class: "bg-white hover:bg-secondary px-40 py-20 gap-10 flex items-center justify-center text-black font-bold text-base rounded-[10px]",
                    onclick: |_| {},
                    {tr.btn_learn}
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

    btn_learn: {
        ko: "$RATEL에 대해 더 알아보기",
        en: "LEARN MORE ABOUT $RATEL",
    },
    btn_join: {
        ko: "지금 참여하기",
        en: "JOIN THE MOVEMENT",
    },

}
