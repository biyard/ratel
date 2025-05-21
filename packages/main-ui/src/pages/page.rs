#![allow(unused)]
use crate::pages::components::{CreateFeedBox, FeedContents, LeftSidebar, RightSidebar};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn IndexPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: IndexTranslate = translate(&lang);

    let my_feeds = ctrl.my_feeds()?;
    let following_feeds = ctrl.following_feeds()?;
    let hot_promotions = ctrl.hot_promotions()?;
    let news = ctrl.news()?;
    let followers = ctrl.followers()?;

    let nickname = ctrl.nickname();
    let profile = ctrl.profile();

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        //FIXME: fix to connect api
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-start items-start py-20 gap-20",
                LeftSidebar { lang }
                FeedContents {
                    lang,
                    my_feeds,
                    following_feeds,
                    profile: profile.clone(),
                }
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

            CreateFeedBox {
                lang,
                nickname,
                profile,
                onsend: move |(content_type, description): (ContentType, String)| async move {
                    ctrl.create_feed(content_type, description).await;
                },
            }
        }
    }
}
