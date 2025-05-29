#![allow(unused)]
use bdk::prelude::*;

use crate::pages::threads::_id::spaces::_id::deliberation::DeliberationController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    nav: Navigator,
    #[allow(dead_code)]
    feed_id: i64,
    #[allow(dead_code)]
    id: i64,

    pub parent_ctrl: DeliberationController,
}

impl Controller {
    pub fn new(lang: Language, feed_id: i64, id: i64) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            nav: use_navigator(),
            feed_id,
            id,

            parent_ctrl: use_context(),
        };

        Ok(ctrl)
    }
}
