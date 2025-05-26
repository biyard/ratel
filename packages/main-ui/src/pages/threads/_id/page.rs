use crate::pages::components::{BottomSheet, LeftSidebar};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ThreadPage(#[props(default = Language::En)] lang: Language, id: i64) -> Element {
    let mut is_write = use_signal(|| false);
    let mut ctrl = Controller::new(lang, id)?;

    let accounts = ctrl.accounts()?;
    let feed = ctrl.feed()?;

    let recent_feeds: Vec<String> = vec![];
    let recent_spaces: Vec<String> = vec![];
    let recent_communities: Vec<String> = vec![];

    let profile_data = ctrl.my_info();

    let tr: ThreadTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { class: "flex flex-col w-full h-screen justify-start items-start",
            div { class: "flex flex-row w-full h-[calc(100vh)] justify-start items-start py-20 px-10 gap-20",
                div { class: "flex flex-row w-fit max-tablet:!hidden",
                    LeftSidebar {
                        lang,
                        profile: profile_data.clone(),
                        // recent_feeds: recent_feeds.clone(),
                        // recent_spaces: recent_spaces.clone(),
                        // recent_communities: recent_communities.clone(),
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

                div { class: "flex flex-row w-full ",
                    Threads {
                        lang,
                        feed,
                        user_id: profile_data.id,
                        create_space: move |_| {
                            ctrl.create_space();
                        },
                        ondownload: move |(name, url): (String, Option<String>)| async move {
                            ctrl.download_file(name, url).await;
                        },
                        onprev: move |_| {
                            ctrl.prev_page();
                        },
                    }
                }
            }

            div {
                class: "fixed bottom-85 left-0 w-full hidden max-tablet:flex aria-hidden:!hidden",
                aria_hidden: is_write(),
                BottomSheet {
                    lang,
                    profile: profile_data.clone(),
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
