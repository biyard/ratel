use crate::pages::threads::_id::spaces::_id::pages::legislation::controller::Controller;
use bdk::prelude::*;

#[component]
pub fn LegislationPage(lang: Language, feed_id: i64, id: i64) -> Element {
    let _ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div { "Legislation Page" }
    }
}
