use crate::pages::threads::_id::spaces::_id::pages::deliberation::controller::Controller;
use bdk::prelude::*;

#[component]
pub fn DeliberationPage(lang: Language, feed_id: i64, id: i64) -> Element {
    let _ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div { "Deliberation Page" }
    }
}
