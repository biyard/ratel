use bdk::prelude::*;

use crate::pages::threads::_id::spaces::_id::pages::deliberation::final_consensus::controller::Controller;

#[component]
pub fn FinalConsensus(
    #[props(default = Language::En)] lang: Language,
    feed_id: i64,
    id: i64,
) -> Element {
    let _ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div { "Final Consensus" }
    }
}
