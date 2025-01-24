#![allow(unused)]
use by_types::QueryParam;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::*;

use crate::route::Route;

use super::NewTopicStep;

#[derive(Clone, Copy)]
pub struct Controller {
    pub size: usize,
    pub topics: Resource<QueryResponse<TopicSummary>>,
    pub status: Signal<Option<TopicStatus>>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let size = 10;
        let status = use_signal(|| None);
        let topic_repository =
            use_signal(|| Topic::get_client(&crate::config::get().main_api_endpoint));

        let topics = use_server_future(move || async move {
            let repo = Topic::get_client(&crate::config::get().main_api_endpoint);
            match repo.query(TopicQuery::new(size)).await {
                Ok(v) => v,
                Err(_) => QueryResponse::default(),
            }
        })?;

        // let items = (list_topics.value())().unwrap_or_default();

        // let topics = use_signal(|| items);

        let ctrl = Self {
            topics,
            size,
            status,
            nav: use_navigator(),
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn get_topics(&self) -> Vec<TopicSummary> {
        self.topics.value()().unwrap_or_default().items
    }

    pub fn navigate_to_create_topic(&self, lang: &Language) {
        self.nav.push(Route::NewTopicPage {
            lang: lang.clone(),
            step: NewTopicStep::SelectLegislation,
            legislation_id: QueryParam::None,
        });
    }
}
