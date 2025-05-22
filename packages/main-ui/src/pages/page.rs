#![allow(unused)]
use crate::pages::{
    components::BottomSheet,
    components::{CreateFeedBox, FeedContents, LeftSidebar, RightSidebar},
};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn IndexPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut is_write = use_signal(|| false);

    let mut ctrl = Controller::new(lang)?;
    let tr: IndexTranslate = translate(&lang);

    let my_feeds = ctrl.my_feeds()?;
    let following_feeds = ctrl.following_feeds()?;
    let hot_promotions = ctrl.hot_promotions()?;
    let news = ctrl.news()?;
    let followers = ctrl.followers()?;
    let profile = ctrl.profile()?;
    let spaces = ctrl.spaces()?;
    let communities = ctrl.communities()?;
    let accounts = ctrl.accounts()?;

    let recent_feeds: Vec<String> = my_feeds
        .iter()
        .map(|v| v.title.clone().unwrap_or_default())
        .take(3)
        .collect();
    let recent_spaces: Vec<String> = spaces
        .iter()
        .map(|v| v.title.clone().unwrap_or_default())
        .take(3)
        .collect();
    let recent_communities: Vec<String> = communities
        .iter()
        .map(|v| v.title.clone().unwrap_or_default())
        .take(3)
        .collect();

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { class: "flex flex-col w-full h-screen justify-start items-start",
            div { class: "flex flex-row w-full h-[calc(100vh)] justify-start items-start py-20 px-10 gap-20",
                div { class: "flex flex-row w-fit max-tablet:!hidden",
                    LeftSidebar {
                        lang,
                        profile: profile.clone(),
                        recent_feeds: recent_feeds.clone(),
                        recent_spaces: recent_spaces.clone(),
                        recent_communities: recent_communities.clone(),
                        accounts: accounts.clone(),

                        onwrite: move |_| {
                            is_write.set(true);
                        },
                        add_account: move |_| async move {
                            ctrl.add_account().await;
                        },
                        sign_out: move |_| {
                            ctrl.signout();
                        },
                    }
                }

                div { class: "flex flex-row w-full ",
                    FeedContents {
                        lang,
                        my_feeds,
                        following_feeds,
                        profile: profile.profile.clone(),

                        is_write: is_write(),
                        onwrite: move |_| {
                            is_write.set(true);
                        },
                    }
                }
                div { class: "flex flex-row w-fit max-tablet:!hidden",
                    RightSidebar {
                        lang,
                        promotion: hot_promotions,
                        news,
                        followers,

                        follow: move |id: i64| async move {
                            ctrl.follow(id).await;
                        },
                    }
                }
            }

            div {
                class: "fixed bottom-85 left-0 w-full hidden max-tablet:flex aria-hidden:!hidden",
                aria_hidden: is_write(),
                BottomSheet {
                    lang,
                    profile: profile.clone(),
                    recent_feeds,
                    recent_spaces,
                    recent_communities,
                    accounts,

                    add_account: move |_| async move {
                        ctrl.add_account().await;
                    },
                    sign_out: move |_| {
                        ctrl.signout();
                    },
                }
            }

            div {
                class: "flex flex-row w-full justify-start items-start aria-hidden:!hidden",
                aria_hidden: !is_write(),
                CreateFeedBox {
                    lang,
                    nickname: profile.nickname.clone(),
                    profile: profile.profile.clone(),
                    onclose: move |_| {
                        is_write.set(false);
                    },
                    onsend: move |(content_type, description): (ContentType, String)| async move {
                        ctrl.create_feed(content_type, description).await;
                    },
                }
            }
        }
    }
}
