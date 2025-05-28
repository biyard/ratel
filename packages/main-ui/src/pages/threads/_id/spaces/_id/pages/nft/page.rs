use crate::pages::threads::_id::spaces::_id::pages::nft::controller::Controller;
use bdk::prelude::*;

#[component]
pub fn NftPage(lang: Language, feed_id: i64, id: i64) -> Element {
    let _ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div { "NFT Page" }
    }
}
