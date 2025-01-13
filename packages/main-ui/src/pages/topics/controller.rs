use dioxus::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Controller {}

impl Controller {
    pub fn new() -> Result<Self, RenderError> {
        let ctrl = Self {};
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}
