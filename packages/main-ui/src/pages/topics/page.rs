#![allow(non_snake_case)]
use crate::components::icons;
use crate::components::icons::RightArrow;
use crate::components::page_title::PageHeader;
use crate::components::page_title::PageHeaderTail;
use crate::route::Route;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::*;

#[component]
pub fn TopicsPage(lang: Language) -> Element {
    let ctrl = Controller::new()?;
    let tr: TopicsTranslate = translate(&lang);

    rsx! {
        div {
            id: "topics",
            class: "flex flex-col justify-start items-start gap-[10px]",
            PageHeader { title: "{tr.title}",
                PageHeaderTail { onclick: move |_| { ctrl.navigate_to_create_topic(&lang) },
                    "{tr.create_topic}"
                    RightArrow {}
                }

            }
            div { class: "w-full flex flex-col justify-start items-start",
                for topic in ctrl.get_topics() {
                    TopicCard { topic, lang }
                }
            }
        }
    }
}

#[component]
pub fn TopicTag(tag: TrendTag) -> Element {
    rsx! {
        div { class: "text-[#202230] text-[10px] font-extrabold uppercase leading-3",
            "{tag}"
        }
    }
}

#[component]
pub fn TopicCard(topic: TopicSummary, lang: Language) -> Element {
    let tr: TopicsCardTranslate = translate(&lang);

    rsx! {
        Link {
            class: "w-full bg-[#404760] rounded-lg flex-col justify-start items-start inline-flex overflow-hidden opacity-80 hover:opacity-100 cursor-pointer",
            to: Route::TopicsByIdPage {
                id: topic.id.clone(),
                lang,
            },
            div { class: "self-stretch px-5 py-2.5 bg-[#404760] rounded-tl-lg rounded-tr-lg border-l border-r border-t border-[#282a3b] justify-start items-start gap-2.5 inline-flex",
                div { class: "grow shrink basis-0 justify-start items-center gap-2.5 flex",
                    div { class: "flex-col justify-start items-start gap-2.5 inline-flex",
                        div { class: "px-1.5 py-1 bg-[#f396c4] rounded justify-center items-center gap-2.5 inline-flex",
                            TopicTag { tag: topic.trend_tag() }
                        }
                        div { class: "h-11 flex-col justify-start items-center gap-1 flex",
                            div { class: "w-[30px] h-[25px] text-center text-white text-2xl font-extrabold uppercase leading-[43.75px]",
                                "{topic.day()}"
                            }
                            div { class: "w-6 h-[15px] text-center text-white text-xs font-extrabold uppercase leading-relaxed",
                                "{topic.month()}"
                            }
                        }
                    }
                    div { class: "grow shrink basis-0 self-stretch justify-start items-start gap-2.5 flex",
                        div { class: "grow shrink basis-0 self-stretch justify-start items-center gap-2 flex",
                            if topic.images.len() >= 1 {
                                img {
                                    src: "{topic.images[0]}",
                                    class: "w-[70px] h-[70px] rounded-md",
                                }
                            }
                            div { class: "grow shrink basis-0 text-white text-base font-extrabold leading-snug tracking-wide",
                                "{topic.title}"
                            }
                        }
                    }
                }
                div { class: "w-[305px] self-stretch justify-start items-center gap-[13px] flex",
                    div { class: "w-[146px] h-[38px] pl-3.5 pr-3 py-2 bg-[#8dff58]/20 rounded-md border border-[#8dff58]/20 flex-col justify-center items-center gap-2.5 inline-flex",
                        div { class: "justify-start items-center gap-2.5 inline-flex",
                            div { class: "text-[#67d36b] text-[15px] font-bold uppercase leading-snug",
                                "{tr.yes}"
                            }
                            icons::OutlinedVoteYes { size: 16 }
                        }
                    }
                    div { class: "w-[146px] h-[38px] pl-3.5 pr-3 py-2 bg-[#ff4145]/20 rounded-md border border-[#ff4145]/20 flex-col justify-center items-center gap-2.5 inline-flex",
                        div { class: "justify-start items-center gap-2.5 inline-flex",
                            div { class: "text-[#ff5a5d] text-[15px] font-bold uppercase leading-snug",
                                "{tr.no}"
                            }
                            icons::OutlinedVoteNo { size: 16 }
                        }
                    }
                }
            }
            div { class: "w-full h-[45px] px-5 py-3 bg-[#404760] rounded-bl-lg rounded-br-lg border border-[#282a3b] flex-col justify-start items-start gap-2.5 flex",
                div { class: "self-stretch justify-between items-end inline-flex",
                    div { class: "justify-start items-center gap-3 flex",
                        div { class: "justify-start items-center gap-1 flex",
                            div { class: "w-5 h-5 relative overflow-hidden",
                                div { class: "w-[15px] h-[15px] left-[2.50px] top-[2.50px] absolute rounded-full border border-[#c4bb7a]" }
                                div { class: "w-2.5 h-[5.83px] left-[5px] top-[7.50px] absolute" }
                            }
                            div { class: "text-[#acbcd6] text-sm font-bold leading-snug",
                                "{tr.vol}"
                            }
                        }
                        div { class: "text-white text-sm font-extrabold uppercase leading-snug",
                            "{topic.volume_with_commas()} {tr.currency}"
                        }
                    }
                    div { class: "justify-start items-center gap-1 flex",
                        div { class: "text-[#acbcd6] text-sm font-bold uppercase leading-snug",
                            "{topic.replies}"
                        }
                    }
                }
            }
        }
    }
}
