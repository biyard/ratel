#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::icons::{
    arrows::ArrowRight,
    emoji::{ThumbsDown, ThumbsUp},
    help_support::Help,
};
use dto::{AssemblyMember, AssemblyMemberSummary};
use politician_card::PoliticianCard;

use crate::{
    components::button::{ButtonSize, secondary_botton::SecondaryButton},
    config,
};

use super::*;

#[component]
pub fn PoliticianStance(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: PoliticianStanceTranslate = translate(&lang);
    let mut selected = use_signal(|| 0);
    let pro_cryptos = use_server_future(move || async move {
        match AssemblyMember::get_client(config::get().main_api_endpoint)
            .list_by_stance(4, None, dto::CryptoStance::Supportive)
            .await
        {
            Ok(members) => members.items,
            _ => {
                // FIXME: change to default after implementing API
                vec![
                    AssemblyMemberSummary {
                        id: 1,
                        name: "John Doe".to_string(),
                        party: "Democratic Party".to_string(),
                        district: "Seoul".to_string(),
                        stance: dto::CryptoStance::Supportive,
                        image_url: "https://www.assembly.go.kr/static/portal/img/openassm/new/e9f57c2b700c44c0845665b068385524.jpg".to_string(),
                        ..Default::default()
                    };4
                ]
            }
        }
    })?;

    let anti_cryptos = use_server_future(move || async move {
        match AssemblyMember::get_client(config::get().main_api_endpoint)
            .list_by_stance(4, None, dto::CryptoStance::Supportive)
            .await
        {
            Ok(members) => members.items,
            _ => {
                // FIXME: change to default after implementing API
                vec![
                    AssemblyMemberSummary {
                        id: 1,
                        name: "John Doe".to_string(),
                        party: "Democratic Party".to_string(),
                        district: "Seoul".to_string(),
                        stance: dto::CryptoStance::Against,
                        image_url: "https://www.assembly.go.kr/static/portal/img/openassm/new/e9f57c2b700c44c0845665b068385524.jpg".to_string(),
                        ..Default::default()
                    };4
                ]
            }
        }
    })?;

    rsx! {
        div {
            id: "politician-stance",
            class: "w-full max-w-1177 h-screen flex flex-col items-start justify-center gap-50 max-[1177px]:mx-10",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }

            div { class: "w-full flex flex-col gap-30 items-center",
                div { class: "w-full flex flex-col gap-10 items-start",
                    div { class: "w-full flex flex-row gap-10 items-start justify-start",
                        ExpandableContainer {
                            tag: tr.pro_crypto,
                            total_count: 10,
                            icon: rsx! {
                                ThumbsUp { class: "[&>path]:stroke-c-c-20", width: "40", height: "40" }
                            },
                            expanded: selected() == 0,
                            onclick: move |_| selected.set(0),

                            div { class: "w-full h-260 grid grid-cols-4 gap-10",
                                for m in pro_cryptos.suspend()?.iter() {
                                    PoliticianCard {
                                        name: "{m.name}",
                                        party: "{m.party}",
                                        image_url: "{m.image_url}",
                                    }
                                }
                            }
                        }

                        ExpandableContainer {
                            tag: tr.anti_crypto,
                            total_count: 10,
                            text_color: "text-c-p-20",
                            icon: rsx! {
                                ThumbsDown { class: "[&>path]:stroke-c-p-20", width: "40", height: "40" }
                            },
                            expanded: selected() == 1,
                            onclick: move |_| selected.set(1),
                            div { class: "w-full h-260 grid grid-cols-4 gap-10",
                                for m in anti_cryptos.suspend()?.iter() {
                                    PoliticianCard {
                                        name: "{m.name}",
                                        party: "{m.party}",
                                        image_url: "{m.image_url}",
                                    }
                                }
                            }
                        }
                    } // end of flex-row

                    div { class: "flex flex-row gap-10 items-center justify-start text-neutral-400 font-medium text-[13px]/18",
                        Help {
                            class: "[&>path]:stroke-neutral-400 [&>circle]:fill-neutral-400",
                            width: "18",
                            height: "18",
                        }
                        span { {tr.legal_notice} }

                    }
                } // end of flex-col

                SecondaryButton { size: ButtonSize::Small,
                    div { class: "flex flex-row gap-10 items-center justify-center font-bold text-sm text-black",
                        {tr.view_all}
                        ArrowRight {
                            class: "[&>path]:stroke-3",
                            width: "15",
                            height: "15",
                        }
                    }
                }

            }
        }
    }
}

translate! {
    PoliticianStanceTranslate;

    title: {
        ko: "Politician Stance",
        en: "Politician Stance",
    },

    mission: {
        ko: "정치인들의 암호화폐 입장은?",
        en: "Where Do Politicians Stand on Crypto?",
    },

    description: {
        ko: "한국의 정치인들이 암호화폐와 블록체인 정책에 대해 어떻게 생각하는지 알아보세요. 이 트래커는 그들의 입법 활동, 공개 발언, 그리고 암호화폐 관련 토론에 참여하는 정도를 분석하여 그들의 입장을 지지적, 중립적, 입장 없음, 정보 없음, 부정적으로 분류합니다.",
        en: "Explore where South Korean lawmakers stand on cryptocurrency and blockchain policy. This tracker analyzes their legislative actions, public statements, and involvement in crypto-related discussions to classify their stance as Supportive, Neutral, No Stance, No Information, or Negative.",
    },

    pro_crypto: {
        ko: "암호화폐 지지",
        en: "Pro-Crypto",
    },

    anti_crypto: {
        ko: "암호화폐 반대",
        en: "Anti-Crypto",
    },

    legal_notice: {
        ko: "초상권 및 법적 고지",
        en: "Portrait Rights & Legal Notice",
    },

    view_all: {
        ko: "전체 보기",
        en: "View All",
    }
}
