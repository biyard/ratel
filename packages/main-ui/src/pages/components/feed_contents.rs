use bdk::prelude::*;

use crate::pages::{components::FeedContent, controller::FeedList};

#[derive(Clone, PartialEq, Translate)]
pub enum Tab {
    #[translate(ko = "For you", en = "For you")]
    Me,
    #[translate(ko = "Following", en = "Following")]
    Following,
}

#[component]
pub fn FeedContents(
    lang: Language,
    profile: String,
    my_feeds: Vec<FeedList>,
    following_feeds: Vec<FeedList>,
) -> Element {
    let mut selected_tab = use_signal(|| Tab::Me);

    rsx! {
        div { class: "flex flex-col w-full h-full justify-start items-start text-white",
            FeedTab {
                lang,
                selected_tab: selected_tab(),
                onchange: move |tab| {
                    selected_tab.set(tab);
                },
            }

            CreateFeed { lang, profile }

            div { class: "flex flex-col w-full h-[calc(100vh-250px)] overflow-y-scroll",

                if selected_tab() == Tab::Me {
                    MyFeedList { lang, my_feeds }
                } else {
                    FollowingFeedList { lang, following_feeds }
                }
            }
        }
    }
}

#[component]
pub fn CreateFeed(lang: Language, profile: String) -> Element {
    let tr: CreateFeedTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-center bg-bg p-20 rounded-lg gap-10 mb-10",
            img { class: "w-36 h-36 rounded-full object-cover", src: profile }
            a {
                class: "flex flex-row w-full h-fit justify-start items-center bg-neutral-800 border border-neutral-700 rounded-[100px] font-normal text-text-secondary text-sm/16 px-15 py-10",
                href: "#create_feed",
                {tr.desc}
            }
        }
    }
}

#[component]
pub fn MyFeedList(lang: Language, my_feeds: Vec<FeedList>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            for feed in my_feeds {
                FeedContent { lang, feed }
            }
        }
    }
}

#[component]
pub fn FollowingFeedList(lang: Language, following_feeds: Vec<FeedList>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            for feed in following_feeds {
                FeedContent { lang, feed }
            }
        }
    }
}

#[component]
pub fn FeedTab(lang: Language, selected_tab: Tab, onchange: EventHandler<Tab>) -> Element {
    let tabs = [Tab::Me, Tab::Following];

    rsx! {
        div { class: "flex flex-row w-full",
            for tab in tabs {
                div {
                    class: "flex flex-col flex-1 items-center cursor-pointer py-4",
                    onclick: {
                        let tab = tab.clone();
                        move |_| {
                            onchange.call(tab.clone());
                        }
                    },

                    div {
                        class: "font-bold text-sm/20 aria-active:text-white text-neutral-400 h-25",
                        "aria-active": selected_tab == tab,
                        {tab.translate(&lang)}
                    }
                    if selected_tab == tab {
                        div { class: "w-29 h-2 mt-1 rounded-full bg-yellow-400" }
                    } else {
                        div { class: "h-2 mt-1" }
                    }
                }
            }
        }
    }
}

translate! {
    CreateFeedTranslate;

    desc: {
        ko: "Discuss legislation. Drive change.",
        en: "Discuss legislation. Drive change."
    }
}
