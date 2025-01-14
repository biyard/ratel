use dioxus_aws::prelude::*;
use dto::{common_query_response::CommonQueryResponse, TopicStatus, TopicSummery};

use crate::services::topic_service::TopicService;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topics: Resource<CommonQueryResponse<TopicSummery>>,
    pub finished_topics: Resource<CommonQueryResponse<TopicSummery>>,
    pub upcoming_topics: Resource<CommonQueryResponse<TopicSummery>>,
}

impl Controller {
    pub fn new() -> Result<Self, RenderError> {
        let topic_api: TopicService = use_context();

        let topics = use_server_future(move || async move {
            match topic_api
                .list_topics_by_status(10, None, Some(TopicStatus::Ongoing))
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    CommonQueryResponse::<TopicSummery>::default()
                }
            }
        })?;

        let finished_topics = use_server_future(move || async move {
            match topic_api
                .list_topics_by_status(10, None, Some(TopicStatus::Finished))
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    CommonQueryResponse::<TopicSummery>::default()
                }
            }
        })?;

        let upcoming_topics = use_server_future(move || async move {
            match topic_api
                .list_topics_by_status(10, None, Some(TopicStatus::Scheduled))
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    CommonQueryResponse::<TopicSummery>::default()
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

    pub fn ongoing_topics(&self) -> Vec<TopicSummery> {
        self.topics.with(|f| {
            tracing::debug!("main topic: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub fn finished_topics(&self) -> Vec<TopicSummery> {
        self.finished_topics.with(|f| {
            tracing::debug!("finished topic: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub fn upcoming_topics(&self) -> Vec<TopicSummery> {
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
