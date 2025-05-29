use bdk::prelude::*;

use crate::route::Route;

use super::PollSettingStep;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    nav: Navigator,
    #[allow(dead_code)]
    pub feed_id: i64,
    #[allow(dead_code)]
    pub id: i64,
    pub is_edit: Signal<bool>,

    pub current_step: Signal<PollSettingStep>,
}

impl Controller {
    pub fn new(lang: Language, feed_id: i64, id: i64) -> std::result::Result<Self, RenderError> {
        let nav = use_navigator();

        let ctrl = Self {
            lang,
            nav,
            feed_id,
            id,
            current_step: use_signal(|| PollSettingStep::Summary),
            is_edit: use_signal(|| false),
        };

        use_effect(move || {
            nav.replace(Route::PollSummary { feed_id, id });
        });

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn change_edit(&mut self, edit: bool) {
        self.is_edit.set(edit);
    }

    pub fn change_current_step(&mut self, step: PollSettingStep) {
        let lang = self.lang;
        let feed_id = self.feed_id;
        let id = self.id;

        tracing::debug!("step: {:?}", step);

        self.current_step.set(step);
        self.nav
            .push(self.current_step().to_route(lang, feed_id, id));
    }
}
