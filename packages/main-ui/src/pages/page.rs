use crate::pages::components::{CreateFeed, MyFeedList};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn IndexPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut ctrl: Controller = use_context();
    let tr: IndexTranslate = translate(&lang);

    let feeds = ctrl.feeds()?;
    let my_info = ctrl.my_info();

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { class: "flex flex-col w-full h-fit justify-start items-start text-white",
            CreateFeed {
                lang,
                profile: my_info.profile_url,
                onwrite: move |_| {
                    ctrl.change_write(true);
                },
            }

            MyFeedList {
                lang,
                feeds,
                my_user_id: my_info.id,
                create_space: move |feed_id: i64| {
                    ctrl.create_space(feed_id);
                },
                add_size: move |_| {
                    ctrl.add_size();
                },
                onclick: move |id: i64| {
                    ctrl.move_to_threads(id);
                },
            }
        }
    }
}
