use bdk::prelude::*;

use crate::pages::threads::_id::spaces::_id::pages::nft::summary::controller::Controller;

#[component]
pub fn NftSummary(
    #[props(default = Language::En)] lang: Language,
    feed_id: i64,
    id: i64,
) -> Element {
    let _ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div { "NFT Summary" }
    }
}
