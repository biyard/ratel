use bdk::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang };
        Ok(ctrl)
    }

    pub fn get_connections(&self) -> Vec<&'static str> {
        vec![
            "Alice Johnson",
            "Bob Smith",
            "Charlie Wang",
            "Debbie Patel",
        ]
    }
}
