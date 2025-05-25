use crate::pages::components::{CreateFeed, MyFeedList};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn IndexPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut is_write = use_signal(|| false);
    let ctrl: Controller = use_context();
    let tr: IndexTranslate = translate(&lang);

    let landing_data = ctrl.landing_data()?;
    let profile = ctrl.profile()?;

    let my_spaces = landing_data.my_spaces;

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { class: "flex flex-col w-full h-full justify-start items-start text-white",
            CreateFeed {
                lang,
                profile: profile.profile,
                onwrite: move |_| {
                    is_write.set(true);
                },
            }

            MyFeedList {
                lang,
                my_spaces,
                onclick: move |id: i64| {
                    ctrl.move_to_threads(id);
                },
            }
        }
    }
}
