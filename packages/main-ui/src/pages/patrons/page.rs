#![allow(non_snake_case)]
use crate::components::icons::DownArrow;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn PatronsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;

    rsx! {
        div { id: "patrons",
            Title { lang }
            Table { lang }
        }
    }
}

#[component]
pub fn Title(lang: Language) -> Element {
    let tr: TitleTranslate = translate(&lang);

    rsx! {
        div { class: "text-[20px] font-semibold", style: "color: white", "{tr.title}" }
    }
}

#[component]
pub fn Table(lang: Language) -> Element {
    rsx! {
        div { class: "h-96 bg-[#414462] rounded-lg flex-col justify-start items-center inline-flex w-full min-h-min mt-[10px]",
            TableMenu { lang }
            Card { lang }
            MoreButton { lang }
        }
    }
}
#[component]
pub fn TableMenu(lang: Language) -> Element {
    let tr: TableMenuTranslate = translate(&lang);

    rsx! {
        div { class: "self-stretch px-[15px] py-[10px] border-b border-[#323342] justify-between items-center inline-flex",
            div {
                class: "w-[150px] justify-start items-center gap-0.5 text-[12px] font-semibold font-['Inter']",
                style: "text-white",
                "{tr.nickname}"
            }
            div {
                class: "w-[100px] justify-end items-end gap-0.5 text-[12px] font-semibold font-['Inter']",
                style: "text-white",
                "{tr.amount}"
            }
            div {
                class: "w-[210px] justify-start items-center gap-0.5 text-[12px] font-semibold font-['Inter']",
                style: "text-white",
                "{tr.proposed_feature}"
            }
            div {
                class: "w-[150px] justify-start items-center gap-0.5 text-[12px] font-semibold font-['Inter']",
                style: "text-white",
                "STATUS"
                "{tr.status}"
            }
        }
    }
}

#[component]
pub fn Card(lang: Language) -> Element {
    rsx! {
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] flex flex-center justify-start items-center gap-2.5",
                img {
                    class: "w-[40px] h-[40px rounded",
                    src: "public/icon/profile.png",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#579dff]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#579dff] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "In PROGRESS"
                    }
                }
            }
        }
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                img {
                    class: "w-10 h-10 rounded",
                    src: "https://via.placeholder.com/40x40",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#323940]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#323940] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "TODO"
                    }
                }
            }
        }
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                img {
                    class: "w-10 h-10 rounded",
                    src: "https://via.placeholder.com/40x40",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#68d36c]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#67d36b] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "DONE"
                    }
                }
            }
        }
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                img {
                    class: "w-10 h-10 rounded",
                    src: "https://via.placeholder.com/40x40",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#68d36c]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#67d36b] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "DONE"
                    }
                }
            }
        }
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                img {
                    class: "w-10 h-10 rounded",
                    src: "https://via.placeholder.com/40x40",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#68d36c]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#67d36b] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "DONE"
                    }
                }
            }
        }
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                img {
                    class: "w-10 h-10 rounded",
                    src: "https://via.placeholder.com/40x40",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#68d36c]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#67d36b] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "DONE"
                    }
                }
            }
        }
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                img {
                    class: "w-10 h-10 rounded",
                    src: "https://via.placeholder.com/40x40",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#68d36c]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#67d36b] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "DONE"
                    }
                }
            }
        }
        div { class: "w-full h-[60px] self-stretch px-3.5 py-2.5 justify-between items-center inline-flex",
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                img {
                    class: "w-10 h-10 rounded",
                    src: "https://via.placeholder.com/40x40",
                }
                div { class: "text-white text-[14px] font-['Inter']", "Gildong Hong" }
            }
            div { class: "w-[100px] justify-end items-end gap-2.5",
                div { class: "text-white text-[14px] font-normal font-['Inter']", "1 ETH" }
            }
            div { class: "w-[210px] justify-start items-center gap-2.5 flex",
                div { class: "px-2.5 py-1 bg-[#323342] rounded justify-start items-center gap-0.5 flex",
                    div { class: "text-white text-sm font-semibold font-['Inter']",
                        "#12"
                    }
                }
            }
            div { class: "w-[150px] justify-start items-center gap-2.5 flex",
                div { class: "h-6 p-2 bg-[#68d36c]/5 rounded-md flex-col justify-center items-center gap-1 inline-flex",
                    div { class: "text-right text-[#67d36b] text-xs font-bold font-['Inter'] uppercase leading-none",
                        "DONE"
                    }
                }
            }
        }
    }
}

#[component]
pub fn MoreButton(lang: Language) -> Element {
    let tr: ButtonTranslate = translate(&lang);

    rsx! {
        div { class: "h-[36px] flex justify-center items-center w-full",
            button {
                onclick: move |_| {
                    tracing::debug!("More button");
                },
                // TODO: Implement loading more items
                class: "h-5 flex justify-center items-center w-full text-white text-sm",
                div { class: "flex items-center gap-3 text-[14px] font-['Inter']",
                    "{tr.more_button}"
                    DownArrow {}
                }
            }
        }
    }
}
