#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::icons::{
    arrows::ArrowRight,
    emoji::{ThumbsDown, ThumbsUp},
    help_support::Help,
};
use dioxus_popup::PopupService;
use dto::AssemblyMember;
use legal_notice_popup::LegalNoticePopup;
use politician_card::PoliticianCard;

use crate::{
    components::{
        button::{ButtonSize, secondary_botton::SecondaryLink},
        icons::BackgroundTriangle,
    },
    config,
    route::Route,
};

use super::*;

#[component]
pub fn PoliticianStance(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tr: PoliticianStanceTranslate = translate(&lang);
    let mut p: PopupService = use_context();

    let mut selected = use_signal(|| 0);
    let pro_cryptos = use_server_future(move || {
        let _ = selected();
        async move {
            match AssemblyMember::get_client(config::get().main_api_endpoint)
                .list_by_stance(4, None, dto::CryptoStance::ProCrypto)
                .await
            {
                Ok(members) => members,
                _ => Default::default(),
            }
        }
    })?
    .suspend()?;

    let anti_cryptos = use_server_future(move || {
        let _ = selected();

        async move {
            match AssemblyMember::get_client(config::get().main_api_endpoint)
                .list_by_stance(4, None, dto::CryptoStance::AntiCrypto)
                .await
            {
                Ok(members) => members,
                _ => Default::default(),
            }
        }
    })?
    .suspend()?;

    rsx! {
        div { class: "hidden max-[900px]:!block w-screen mt-30",
            BackgroundTriangle { color: "#1E1E1E" }
        }
        div {
            id: "politician-stance",
            class: "w-screen max-w-1177 h-full flex flex-col items-start justify-center mt-150 gap-50 max-[1177px]:mx-10 max-[900px]:px-30 max-[900px]:!mt-0",
            SectionHeader {
                section_name: tr.title,
                title: tr.mission,
                description: tr.description,
            }
            div { class: "hidden max-[900px]:!block",
                SecondaryLink {
                    size: ButtonSize::Small,
                    to: Route::PoliticiansPage { lang },
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


            div { class: "w-full flex flex-col gap-30 items-center",
                div { class: "w-full flex flex-col gap-10 items-start",
                    //desktop_pro
                    div { class: "block max-[900px]:!hidden w-full flex flex-row gap-10",
                        ExpandableContainer {
                            tag: tr.pro_crypto,
                            total_count: pro_cryptos().total_count,
                            icon: rsx! {
                                ThumbsUp { class: "[&>path]:stroke-c-c-20", width: "40", height: "40" }
                            },
                            expanded: selected() == 0,
                            onclick: move |_| {
                                tracing::debug!("selected: 0");
                                selected.set(0);
                            },
                            div { class: "w-full h-260 grid grid-cols-4 gap-10",
                                for m in pro_cryptos().items {
                                    PoliticianCard {
                                        lang,
                                        id: m.id,
                                        name: "{m.name}",
                                        party: m.party_enum(),
                                        image_url: "{m.image_url}",
                                    }
                                }
                            }
                        }
                        //desktop_anti
                        ExpandableContainer {
                            tag: tr.anti_crypto,
                            total_count: anti_cryptos().total_count,
                            text_color: "text-c-p-20",
                            icon: rsx! {
                                ThumbsDown { class: "[&>path]:stroke-c-p-20", width: "40", height: "40" }
                            },
                            expanded: selected() == 1,
                            onclick: move |_| {
                                tracing::debug!("selected: 1");
                                selected.set(1);
                            },
                            div { class: "w-full h-260 grid grid-cols-4 gap-10",
                                for m in anti_cryptos().items {
                                    PoliticianCard {
                                        lang,
                                        id: m.id,
                                        name: "{m.name}",
                                        party: m.party_enum(),
                                        image_url: "{m.image_url}",
                                    }
                                }
                            }
                        }
                    }

                    //TODO(web): make scroll works and build UI
                    //mobile_pro
                    div { class: "hidden max-[900px]:!block w-full h-full",
                        ExpandableContainer {
                            tag: tr.pro_crypto,
                            total_count: pro_cryptos().total_count,
                            icon: rsx! {
                                ThumbsUp { class: "[&>path]:stroke-c-c-20", width: "40", height: "40" }
                            },
                            expanded: selected() == 0,
                            onclick: move |_| {
                                tracing::debug!("selected: pro");
                            },
                            div { class: "w-full h-260 flex flex-row gap-10 overflow-x-auto scroll-smooth",
                                for m in pro_cryptos().items {
                                    PoliticianCard {
                                        lang,
                                        id: m.id,
                                        name: "{m.name}",
                                        party: m.party_enum(),
                                        image_url: "{m.image_url}",
                                    
                                    }
                                }
                            }
                        }
                        //mobile_anti
                        ExpandableContainer {
                            tag: tr.anti_crypto,
                            total_count: anti_cryptos().total_count,
                            text_color: "text-c-p-20",
                            icon: rsx! {
                                ThumbsDown { class: "[&>path]:stroke-c-p-20", width: "40", height: "40" }
                            },
                            expanded: selected() == 0,
                            onclick: move |_| {
                                tracing::debug!("selected: anti");
                            },
                            div { class: "w-full h-260 flex flex-row gap-10 overflow-x-auto scroll-smooth",
                                for m in anti_cryptos().items {
                                    PoliticianCard {
                                        lang,
                                        id: m.id,
                                        name: "{m.name}",
                                        party: m.party_enum(),
                                        image_url: "{m.image_url}",
                                    }
                                }
                            }
                        }
                    } // end of flex-row

                    div {
                        class: "flex flex-row gap-10 items-center justify-start text-neutral-400 font-medium text-[13px]/18 tooltip cursor-pointer",
                        "data-tip": tr.legal,
                        onclick: move |_| {
                            p.open(rsx! {
                                LegalNoticePopup { lang }
                            });
                        },
                        Help {
                            class: "[&>path]:stroke-neutral-400 [&>circle]:fill-neutral-400",
                            width: "18",
                            height: "18",
                        }
                        span { {tr.legal_notice} }
                    }
                } // end of flex-col

                div { class: "block max-[900px]:!hidden",
                    SecondaryLink {
                        size: ButtonSize::Small,
                        to: Route::PoliticiansPage { lang },
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

    legal: {
        ko: "이 사이트에서 제공하는 의원 정보는 국회의원 공개 데이터를 기반으로 합니다. 정보를 최신 상태로 유지하기 위해 노력하고 있습니다. 오류나 변경 사항을 발견하면 연락 주시면 즉시 수정하겠습니다.",
        en: "The information on legislators provided on this site is based on publicly available data from the National Assembly. We strive to keep the information up to date. If you notice any errors or changes, please contact us, and we will correct them promptly.",
    }
}
