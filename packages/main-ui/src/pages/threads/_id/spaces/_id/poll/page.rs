use crate::pages::threads::_id::spaces::_id::poll::controller::Controller;
use bdk::prelude::*;

#[component]
pub fn PollPage(lang: Language, feed_id: i64, id: i64) -> Element {
    let _ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div { "Poll Page" }
    }
}
