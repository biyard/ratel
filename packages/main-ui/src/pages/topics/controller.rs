#![allow(unused)]
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::*;

use crate::route::Route;

#[derive(Clone, Copy)]
pub struct Controller {
    pub size: usize,
    pub topics: Signal<Vec<TopicSummary>>,
    pub bookmark: Signal<Option<String>>,
    pub status: Signal<Option<TopicStatus>>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let size = 10;
        let status = use_signal(|| None);
        let topic_repository =
            use_signal(|| Topic::get_client(&crate::config::get().main_api_endpoint));

        let list_topics = use_server_future(move || async move {
            let repo = Topic::get_client(&crate::config::get().main_api_endpoint);
            match repo.query(TopicQuery::new(size)).await {
                Ok(v) => v,
                Err(_) => CommonQueryResponse::default(),
            }
        })?;

        let CommonQueryResponse::<TopicSummary> { items, bookmark } =
            (list_topics.value())().unwrap_or_default();

        let topics = use_signal(|| items);
        let bookmark = use_signal(|| bookmark);

        let ctrl = Self {
            topics,
            bookmark,
            size,
            status,
            nav: use_navigator(),
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn navigate_to_create_topic(&self, lang: &Language) {
        self.nav.push(Route::NewTopicPage { lang: lang.clone() });
    }
}
