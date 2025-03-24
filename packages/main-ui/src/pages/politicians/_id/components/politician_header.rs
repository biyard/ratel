#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::icons::{
    email::Email,
    emoji::{ThumbsDown, ThumbsUp},
};
use dto::{CryptoStance, Party};

use crate::pages::politicians::components::party::PartyIcon;

#[component]
pub fn PoliticianHeader(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    image: String,
    name: String,
    party: Party,
    stance: CryptoStance,
    email: String,
    description: String,

    children: Element,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-row gap-24",
            img { class: "w-233 h-260 rounded-[10px] object-cover", src: image }

            div { class: "grow flex flex-col justify-between",

                div { id: "politician-info", class: "flex flex-col gap-24",
                    h1 { class: "text-[32px]/40 font-bold text-text-primary", {name} }

                    div {
                        id: "politician-badges",
                        class: "flex flex-col gap-13 text-text-primary font-medium text-[15px]",

                        div { class: "flex flex-row gap-8",
                            PartyIcon { party }
                            {party.translate(&lang)}
                        }

                        div { class: "flex flex-row gap-8",
                            Email {
                                class: "[&>path]:stroke-white [&>rect]:stroke-white",
                                width: "18",
                                height: "18",
                            }
                            span { {email} }
                        }
                    } // politician-badges
                } // politician-info

                div {
                    id: "politician-header-crypto-stance",
                    class: "w-full rounded-[20px] bg-bg py-24 px-24 flex flex-col gap-5 text-lg/22 font-bold text-text-primary",
                    div { class: "flex flex-row gap-10 items-center",
                        CryptoStanceIcon { stance }
                        {stance.translate(&lang)}
                    }
                }
            }

        }
    }
}

#[component]
pub fn CryptoStanceIcon(stance: CryptoStance) -> Element {
    rsx! {
        if stance == CryptoStance::ProCrypto {
            ThumbsUp { class: "[&>path]:stroke-c-c-20", width: "24", height: "24" }
        } else if stance == CryptoStance::AntiCrypto {
            ThumbsDown { class: "[&>path]:stroke-c-p-20", width: "24", height: "24" }
        }
    }
}
