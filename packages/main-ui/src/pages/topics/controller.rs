use dioxus::prelude::*;
use dto::*;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topics: Resource<Vec<Topic>>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let topics = use_server_future(move || async move { vec![] })?;
        let ctrl = Self { topics };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}
