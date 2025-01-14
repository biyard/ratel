#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;
use dioxus_popup::PopupService;
use crate::{
    theme::Theme,
    components::{
        icons,
        tooltip::Tooltip,
    },
};
use dto::{CryptoStance, District};
use super::{
    filter_popup::FilterPopup, 
    email_verification_popup::EmailVerificationPopup,
    i18n::PoliticianStanceTranslate,
};

#[component]
pub fn PoliticianStatusPage(lang: Language) -> Element {
    let tr = translate::<PoliticianStanceTranslate>(&lang);
    rsx! {
        div { class: "flex flex-col justify-start w-full min-h-[100vh] text-white max-[1440px]:px-[10px] gap-[10px]",
            div {
                class: "text-xl font-semibold text-white",
                "{tr.title}"
            },
            PoliticianStatusTable { lang: lang }
        }
    }
}

#[component]
pub fn PoliticianStatusTable(lang: Language) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr: PoliticianStanceTranslate = translate(&lang);
    let ctrl = super::controller::Controller::new(lang)?;
    // TODO: mobile view
    rsx! {
        div { class: "w-full h-full flex flex-col bg-[{theme.primary06}] rounded-[8px] text-white",
            PoliticianStatusHeader { lang }
            div { class: "w-full h-full flex flex-col gap-[10px]",
                for politician in ctrl.politicians() {
                    PoliticianStatusRow {
                        lang: lang.clone(),
                        name: politician.name,
                        party: politician.party,
                        district: politician.district,
                        image: politician.image_url,
                        // TODO: replace with real data
                        stance: CryptoStance::NoStance,
                    }
                }
            }
            // TODO: 다음 10개 미리 떙겨놓고 값 없으면 hide
            PoliticianStatusMoreRow { text: tr.more }
        }
    }
}

#[component]
pub fn PoliticianStatusHeader(lang: Language) -> Element {
    let tr = translate::<PoliticianStanceTranslate>(&lang);
    let mut popup: PopupService = use_context();

    let onclick = move |_: Event<MouseData>| {
        tracing::debug!("header clicked");
        popup
            .open(rsx! {
                FilterPopup {
                    class: "w-[452px]",
                    lang: lang.clone(),
                }
            })
            .with_id("politician_status_filter_popup")
            .with_title(tr.search_title);
    };

    rsx! {
        div { class: "w-full flex flex-row items-center gap-[90px] px-[15px] py-[10px] border-b-[1px] border-[#323342]",
            onclick,
            div { class: "flex items-center w-[280px] gap-[2px]", 
                span {
                    class: "text-xs font-semibold",
                    "{tr.name}"
                }
                icons::Search { color: "white" }
            }
            div { class: "flex items-center w-[150px] gap-[2px]", 
                span {
                    class: "text-xs font-semibold",
                    "{tr.party}"
                }
                icons::Sort { color: "white", filled: false }
            }
            div { class: "flex items-center w-[200px] gap-[2px]", 
                span {
                    class: "text-xs font-semibold",
                    "{tr.district}"
                }
                icons::Sort { color: "white", filled: true }
            }
            div { class: "flex items-center w-[210px] gap-[2px]", 
                span {
                    class: "text-xs font-semibold",
                    "{tr.stance_on_crypto}"
                }
                icons::Sort { color: "white", filled: false }
            }
            div { class: "flex items-start w-[210px] gap-[1px]", 
                span {
                    class: "text-xs font-semibold",
                    "{tr.proclaim}"
                }
                Tooltip {
                    inner_class: "text-xs text-white font-bold bg-[#2C2E42] px-[15px] py-[10px] rounded-[8px] shadow-2xl w-[230px] h-[80px]".to_string(),
                    text: "{tr.tooltip}",
                    bg_color: "#2C2E42".to_string(),
                    icons::Tooltip { color: "#ADBCD7" }
                }
            }
        }
    }
}

#[component]
pub fn PoliticianStatusRow(
    lang: Language,
    #[props(default = "-".to_string())] name: String,
    #[props(default = "-".to_string())] party: String,
    #[props(default = District::default())] district: District,
    // FIXME: replace with default image url
    #[props(default = "https://s3-alpha-sig.figma.com/img/1656/3e71/c59ce479012efb94f2c8e2de7e8edb01?Expires=1737331200&Key-Pair-Id=APKAQ4GOSFWCVNEHN3O4&Signature=eAYKpzVwViWK-SS69oVrWA7uXV19jcw1kNTCYqwyVTH8ZSb6X5MiPGEdtzMMIcsSibtPZn4HcMI8~GkegFgoxMMTL46Q3yhlyNWcYBhB6JAeOYP48igQbIhqJQDPhF3VLpobYfwkMlhFbwIHVaT5m0~HWSB7-pUUZduDGDkKFZ0UZeoxJPbHFopGJB1AZplTRwm4xV9veeHFKyaWxjMY~JidYJeyCz5Rloq1nXOJ2ma3RSU-BKRjuZgpEybj0dRXEyC2wz1oh9V1sQmciKNVAKUGk9X~Fm2xiA9qjx81KLlPvvM0QmwS5q3t9N21CcneNyBKe4y2MnAE-HIdksIC5A__".to_string())] image: String,
    #[props(default = CryptoStance::NoStance )] stance: CryptoStance,
) -> Element {
    let theme_service: Theme = use_context();
    let theme: crate::theme::ThemeData = theme_service.get_data();
    let tr: PoliticianStanceTranslate = translate(&lang);
    let mut popup: PopupService = use_context();

    // TODO: feat oppacity on hover
    rsx! {
        div { class: "w-full h-[60px] px-[15px] py-[10px] flex flex-row items-center justify-start gap-[90px] hover:bg-[#32334280]",
            div { class: "flex text-sm font-semibold w-[280px] gap-[10px] items-center",
                img { 
                    class: "w-[40px] h-[40px] rounded-[5px] object-cover", 
                    src: image,
                },
                "{name}"
            }
            div { class: "text-sm w-[150px]", "{party}" }
            div { class: "text-sm w-[200px]", "{district}" }
            div { class: "flex items-center text-sm w-[210px] gap-[10px]", 
                if stance == CryptoStance::Supportive {
                    icons::Pros { color: "{theme.active00}" },
                    "{tr.supportive}"
                } else if stance == CryptoStance::Against {
                    icons::Cons { color: "{theme.active_false}" },
                    "{tr.against}"
                } else if stance == CryptoStance::Neutral {
                    icons::HandPalm {} ,
                    "{tr.neutral}"
                } else {
                    "{tr.no_stance}"
                } 
            }
            div { class: "px-[10px] py-[5px] bg-[{theme.hover}] rounded-[5px]", 
                button { class: "text-sm font-semibold",
                    onclick: move |_| {
                        tracing::debug!("proclaim clicked");
                        // TODO: check if email is null
                        // TODO: replace dummy data
                        popup
                            .open(rsx! {
                                EmailVerificationPopup {
                                    class: "w-[450px]",
                                    name: "gildong hong".to_string(),
                                    party: "people power party".to_string(),
                                    email: "test@test.com".to_string(),
                                    stance: CryptoStance::default(),
                                    lang: lang.clone(),
                                }
                            })
                            .with_id("email_verification_popup")
                            .with_title(tr.stance_on_crypto);
                    },
                    "# {tr.change_stance}" 
                }
            }
        }
    }
}

#[component]
pub fn PoliticianStatusMoreRow(text: String) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();

    rsx!(
        button {
            class: "w-full h-[36px] flex flex-row items-center justify-center gap-[10px] hover:bg-[{theme.hover}] transition-colors",
            span {
                class: "text-sm",
                "{text}"
            }
            icons::DoubleArrowDown { color: "white" }
        }
    )
}