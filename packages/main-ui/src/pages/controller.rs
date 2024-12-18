use dioxus_aws::prelude::*;
use dto::{common_query_response::CommonQueryResponse, Topic, TopicStatus};

use crate::services::topic_api::TopicApi;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topics: Resource<CommonQueryResponse<Topic>>,
    pub finished_topics: Resource<CommonQueryResponse<Topic>>,
    pub upcoming_topics: Resource<CommonQueryResponse<Topic>>,
}

impl Controller {
    pub fn new() -> Result<Self, RenderError> {
        let topic_api: TopicApi = use_context();
        let topics = use_server_future(move || async move {
            match topic_api
                .list_topics(10, None, Some(TopicStatus::Ongoing))
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    CommonQueryResponse::<Topic>::default()
                }
            }
        })?;

        let finished_topics = use_server_future(move || async move {
            match topic_api
                .list_topics(10, None, Some(TopicStatus::Finished))
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    CommonQueryResponse::<Topic>::default()
                }
            }
        })?;

        let upcoming_topics = use_server_future(move || async move {
            match topic_api
                .list_topics(10, None, Some(TopicStatus::Scheduled))
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    CommonQueryResponse::<Topic>::default()
                }
            }
        })?;

        let ctrl = Self {
            topics,
            finished_topics,
            upcoming_topics,
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn ongoing_topics(&self) -> Vec<Topic> {
        self.topics.with(|f| {
            tracing::debug!("main topic: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub fn finished_topics(&self) -> Vec<Topic> {
        self.finished_topics.with(|f| {
            tracing::debug!("finished topic: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub fn upcoming_topics(&self) -> Vec<Topic> {
        self.upcoming_topics.with(|f| {
            tracing::debug!("upcoming topic: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                vec![]
            }
        })
    }
}
