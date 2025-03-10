#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

use super::*;

#[component]
pub fn Community(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: CommunityTranslate = translate(&lang);

    rsx! {
        div {
            id: "community",
            class: "w-full max-w-1177 h-screen flex flex-col items-start justify-center gap-50 max-[1177px]:mx-10",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }

            "Form"
        }
    }
}

translate! {
    CommunityTranslate;

    title: {
        ko: "Community",
        en: "Community",
    }

    mission: {
        ko: "Ratel DAO: 탈중앙화된 거버넌스 허브",
        en: "Ratel DAO: Decentralized Governance Hub",
    }

    description: {
        ko: "Ratel DAO에 가입하여 완전히 탈중앙화되고 투명한 생태계에서 기여하고 투표하며 주요 결정에 영향을 미치세요.",
        en: "Join Ratel DAO to contribute, vote, and influence major decisions in a fully decentralized and transparent ecosystem.",
    }
}
