use bdk::prelude::*;

use crate::pages::threads::_id::spaces::_id::pages::deliberation::summary::controller::Controller;

#[component]
pub fn Summary(#[props(default = Language::En)] lang: Language, feed_id: i64, id: i64) -> Element {
    let _ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div { {format!("Summary")} }
    }
}
