#![allow(unused)]
use bdk::prelude::*;

use crate::route::Route;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    nav: Navigator,
    #[allow(dead_code)]
    pub feed_id: i64,
    #[allow(dead_code)]
    pub id: i64,

    pub is_edit: Signal<bool>,
}

impl Controller {
    pub fn new(lang: Language, feed_id: i64, id: i64) -> std::result::Result<Self, RenderError> {
        tracing::debug!("11111");
        let nav = use_navigator();

        let ctrl = Self {
            lang,
            nav,
            feed_id,
            id,
            is_edit: use_signal(|| false),
        };

        use_context_provider(|| ctrl);

        use_effect(move || {
            nav.replace(Route::LegislationSummary { feed_id, id });
        });

        Ok(ctrl)
    }

    pub fn change_edit(&mut self, edit: bool) {
        self.is_edit.set(edit);
    }
}
