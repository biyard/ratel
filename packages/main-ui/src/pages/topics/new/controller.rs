use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::TopicCreateRequest;

#[derive(Debug, Clone, Copy)]
pub struct Controller {}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {};
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub async fn handle_create_topic(&self, req: TopicCreateRequest) {
        tracing::debug!("Creating topic: {:?}", req);
    }

    pub fn navigate_to_write_topic(&self, lang: &Language, legisltation: Option<String>) {
        tracing::debug!(
            "Navigating to write topic page with legislation: {:?} {:?}",
            lang,
            legisltation
        );
    }
}
