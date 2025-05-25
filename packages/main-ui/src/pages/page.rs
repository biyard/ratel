use crate::pages::components::FeedContents;

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
    let following_spaces = landing_data.following_spaces;

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        FeedContents {
            lang,
            my_spaces,
            following_spaces,
            profile: profile.profile.clone(),

            is_write: is_write(),
            onwrite: move |_| {
                is_write.set(true);
            },
            onclick: move |id: i64| {
                ctrl.move_to_threads(id);
            },
        }
    }
}
