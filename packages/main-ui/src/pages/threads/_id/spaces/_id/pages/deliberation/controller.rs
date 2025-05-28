#![allow(unused)]
use bdk::prelude::*;

use crate::route::Route;

use super::DeliberationSettingStep;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    nav: Navigator,
    #[allow(dead_code)]
    feed_id: i64,
    #[allow(dead_code)]
    id: i64,
    pub is_edit: Signal<bool>,

    pub current_step: Signal<DeliberationSettingStep>,
}

impl Controller {
    pub fn new(lang: Language, feed_id: i64, id: i64) -> std::result::Result<Self, RenderError> {
        let nav = use_navigator();

        let ctrl = Self {
            lang,
            nav,
            feed_id,
            id,
            current_step: use_signal(|| DeliberationSettingStep::Summary),
            is_edit: use_signal(|| false),
        };

        use_effect(move || {
            nav.replace(Route::Summary { feed_id, id });
        });

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn change_edit(&mut self, edit: bool) {
        self.is_edit.set(edit);
    }

    pub fn change_current_step(&mut self, step: DeliberationSettingStep) {
        let lang = self.lang;
        let feed_id = self.feed_id;
        let id = self.id;

        tracing::debug!("step: {:?}", step);

        self.current_step.set(step);
        self.nav
            .push(self.current_step().to_route(lang, feed_id, id));
    }
}
