#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::icons::{
    email::Email,
    emoji::{ThumbsDown, ThumbsUp},
};
use dto::{CryptoStance, Party};

use crate::components::party::PartyIcon;

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
        div { class: "w-full flex flex-row gap-24 max-mobile:!flex-col max-mobile:!gap-10 max-mobile:!items-center max-mobile:!justify-center",
            img { class: "w-233 h-260 rounded-[10px] object-cover", src: image }

            div { class: "grow flex flex-col justify-between",
                div {
                    id: "politician-info",
                    class: "grow flex flex-col gap-24 max-mobile:!grow",
                    h1 { class: "text-[32px]/40 font-bold text-text-primary max-mobile:flex max-mobile:items-center max-mobile:justify-center",
                        {name}
                    }

                    div { class: "grow flex flex-col justify-between max-mobile:gap-13",
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

                        div {
                            id: "politician-header-crypto-stance",
                            class: "w-full rounded-[20px] bg-bg py-24 px-24 flex flex-col gap-5 text-lg/22 font-bold text-text-primary max-mobile:!p-0 max-mobile:!bg-transparent max-mobile:text-[15px]",
                            div { class: "flex flex-row gap-10 items-center",
                                div { class: "max-mobile:hidden",
                                    CryptoStanceIcon { stance }
                                }
                                div { class: "hidden max-mobile:!block",
                                    CryptoStanceIcon { size: 18, stance }
                                }

                                {stance.translate(&lang)}
                            }

                        }
                    }

                } // politician-info

            }
        }
    }
}

#[component]
pub fn CryptoStanceIcon(#[props(default = 24)] size: i32, stance: CryptoStance) -> Element {
    rsx! {
        if stance == CryptoStance::Supportive || stance == CryptoStance::StronglySupportive {
            ThumbsUp {
                class: "[&>path]:stroke-c-c-20",
                width: "{size}",
                height: "{size}",
            }
        } else if stance == CryptoStance::Against || stance == CryptoStance::StronglyAgainst {
            ThumbsDown {
                class: "[&>path]:stroke-c-p-20",
                width: "{size}",
                height: "{size}",
            }
        }
    }
}
