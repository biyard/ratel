#![allow(unused)]
use dioxus::prelude::*;
use dto::*;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub size: usize,
    pub topics: Signal<Vec<TopicSummery>>,
    pub bookmark: Signal<Option<String>>,
    pub status: Signal<Option<TopicStatus>>,
    pub topic_repository: Signal<TopicClient>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let size = 10;
        let status = use_signal(|| None);
        let topic_repository =
            use_signal(|| TopicSummery::get_client(crate::config::get().main_api_endpoint.clone()));

        let list_topics = use_server_future(move || async move {
            match topic_repository()
                .query(TopicQuery {
                    size,
                    bookmark: None,
                    status: None,
                })
                .await
            {
                Ok(v) => v,
                Err(_) => CommonQueryResponse::default(),
            }
        })?;
        let CommonQueryResponse::<TopicSummery> { items, bookmark } =
            (list_topics.value())().unwrap_or_default();

        let topics = use_signal(|| items);
        let bookmark = use_signal(|| bookmark);

        let ctrl = Self {
            topics,
            bookmark,
            size,
            status,
            topic_repository,
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}
