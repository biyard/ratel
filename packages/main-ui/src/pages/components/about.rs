#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

use crate::{components::indicators::Indicator, pages::components::mission_card::MissionCard};

#[component]
pub fn About(lang: Language) -> Element {
    let tr: AboutTranslate = translate(&lang);

    rsx! {
        div {
            id: "about",
            class: "w-full max-w-1177 h-screen flex flex-col items-start justify-center gap-50 max-[1177px]:mx-10",
            div { class: "w-full flex flex-col justify-start items-start gap-20",
                Indicator { {tr.title} }
                div { class: "w-full grid grid-cols-2 gap-24",
                    h1 { class: "w-full col-span-1 text-[32px] font-bold text-white",
                        {tr.mission}
                    }
                    div { class: "col-span-1 w-full h-full flex flex-col items-center justify-center",
                        div { class: "w-full h-1 bg-[#464646]" }
                    }
                    p { class: "col-span-1 w-full text-white font-normal text-[15px]/22 text-[#AEAEAE] whitespace-pre-line",
                        {tr.description}
                    }
                }
            }

            div { class: "w-full grid grid-cols-4 gap-24",
                MissionCard {
                    class: "col-span-1 w-full h-352",
                    no: "01.",
                    title: tr.mission1,
                    description: tr.mission1_description,
                }
                MissionCard {
                    class: "col-span-1 w-full h-352",
                    no: "02.",
                    title: tr.mission2,
                    description: tr.mission2_description,
                }
                MissionCard {
                    class: "col-span-1 w-full h-352",
                    no: "03.",
                    title: tr.mission3,
                    description: tr.mission3_description,
                }
                MissionCard {
                    class: "col-span-1 w-full h-352",
                    no: "04.",
                    title: tr.mission4,
                    description: tr.mission4_description,
                }
            }
        }
    }
}

translate! {
    AboutTranslate;

    title: {
        ko: "About",
        en: "About",
    },

    mission: {
        ko: "미션",
        en: "Our Mission",
    },

    description: {
        ko: "Ratel은 공정하고 투명한 암호화폐 규제를 주장하는 탈중앙화 거버넌스 프로젝트입니다. 커뮤니티를 능력있게 하는 것으로 정책 결정자와 블록체인 산업 간의 간극을 줄이고, 정책이 혁신을 촉진하도록 보장합니다.",
        en: "Ratel is a decentralized governance project advocating for fair and transparent crypto regulations. By empowering communities, we bridge the gap between policymakers and the blockchain industry, ensuring policies foster innovation rather than hinder progress.",
    }

    mission1: {
        ko: "",
        en:"Decentralized Governance (DAO)",
    }
    mission1_description: {
        ko: "",
        en:"The Ratel DAO enables anyone to propose and vote on crypto policy initiatives.",
    }


    mission2: {
        ko: "",
        en:"Politician Stance Tracking",
    }

    mission2_description: {
        ko: "",
        en:"We analyze lawmakers' positions on crypto and ensure accountability.",
    }

    mission3: {
        ko: "",
        en:"Community-Driven Advocacy",
    }

    mission3_description: {
        ko: "",
        en:"Ratel funds initiatives that support regulatory clarity and industry-friendly policies.",
    }

    mission4: {
        ko: "",
        en:"Community Participation-Based Token Distribution",
    }

    mission4_description: {
        ko: "",
        en:"Supporters receive RATEL tokens in return for their contributions, allowing them to participate in governance.",
    }
}
