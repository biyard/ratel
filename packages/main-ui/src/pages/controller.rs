use dioxus_aws::prelude::*;
use dto::{common_query_response::CommonQueryResponse, Topic};

use crate::services::topic_api::TopicApi;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topics: Resource<CommonQueryResponse<Topic>>,
    pub topic_api: TopicApi,
}

impl Controller {
    pub fn new() -> Result<Self, RenderError> {
        let topic_api: TopicApi = use_context();
        let topics = use_server_future(move || async move {
            match topic_api.list_topics(10, None).await {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    CommonQueryResponse::<Topic>::default()
                }
            }
        })?;
        let ctrl = Self { topics, topic_api };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn main_topic(&self) -> Vec<Topic> {
        self.topics.with(|f| {
            tracing::debug!("main topic: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                vec![]
            }
        })
    }
}
