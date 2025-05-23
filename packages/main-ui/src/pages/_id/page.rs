use crate::pages::components::{BottomSheet, LeftSidebar};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ThreadPage(#[props(default = Language::En)] lang: Language, id: i64) -> Element {
    let mut is_write = use_signal(|| false);
    let mut ctrl = Controller::new(lang)?;

    let profile = ctrl.profile()?;
    let accounts = ctrl.accounts()?;
    let my_feeds = ctrl.my_feeds()?;
    let spaces = ctrl.spaces()?;
    let communities = ctrl.communities()?;

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

    let tr: ThreadTranslate = translate(&lang);

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
                        sign_out: move |_| async move {
                            ctrl.signout().await;
                        },
                    }
                }

                div { class: "flex flex-row w-full ", {"Hello"} }
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
                    sign_out: move |_| async move {
                        ctrl.signout().await;
                    },
                }
            }
        }
    }
}
