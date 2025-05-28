use bdk::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub feed_id: i64,
    #[allow(dead_code)]
    pub id: i64,
}

impl Controller {
    pub fn new(lang: Language, feed_id: i64, id: i64) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang, feed_id, id };

        Ok(ctrl)
    }
}
