#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::components::icons::BackgroundTriangle;

use super::*;

#[component]
pub fn About(lang: Language) -> Element {
    rsx! {
        div { class: "hidden md:!block",
            DesktopAbout { lang }
        }
        div { class: "block md:!hidden",
            MobileAbout { lang }
        }
    }
}

#[component]
pub fn DesktopAbout(lang: Language) -> Element {
    let tr: AboutTranslate = translate(&lang);

    rsx! {
        div {
            id: "about",
            class: "w-full max-w-1177 h-screen flex flex-col items-start justify-center gap-50 max-[1177px]:mx-10",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
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

#[component]
pub fn MobileAbout(lang: Language) -> Element {
    let tr: AboutTranslate = translate(&lang);

    rsx! {
        div {
            id: "mobile_about",
            class: "w-screen h-full flex flex-col justify-center items-center",
            div {
                BackgroundTriangle { color: "#1E1E1E" }
            }
            div { class: "w-screen h-full px-[30px] bg-[#1e1e1e] flex flex-col items-center justify-center gap-[40px]",
                MobileSectionHeader {
                    section_name: tr.title,
                    title: tr.mission,
                    description: tr.description,
                }

                div { class: "w-full flex flex-col justify-center items-center gap-[10px]",
                    MobileMissionCard {
                        class: "w-full min-w-[300px] h-auto row-span-1",
                        no: "01.",
                        title: tr.mission1,
                        description: tr.mission1_description,
                    }
                    MobileMissionCard {
                        class: "w-full min-w-[300px] h-auto row-span-2",
                        no: "02.",
                        title: tr.mission2,
                        description: tr.mission2_description,
                    }
                    MobileMissionCard {
                        class: "w-full min-w-[300px] h-auto row-span-3",
                        no: "03.",
                        title: tr.mission3,
                        description: tr.mission3_description,
                    }
                    MobileMissionCard {
                        class: "w-full min-w-[300px] h-auto row-span-4",
                        no: "04.",
                        title: tr.mission4,
                        description: tr.mission4_description,
                    }
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
