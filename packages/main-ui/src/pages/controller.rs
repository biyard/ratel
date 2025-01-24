use dioxus_aws::prelude::*;
use dto::{QueryResponse, Topic, TopicQuery, TopicStatus, TopicSummary};

use crate::config;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topics: Resource<QueryResponse<TopicSummary>>,
    pub finished_topics: Resource<QueryResponse<TopicSummary>>,
    pub upcoming_topics: Resource<QueryResponse<TopicSummary>>,
}

impl Controller {
    pub fn new() -> Result<Self, RenderError> {
        let conf = config::get();

        let topics = use_server_future(move || async move {
            let topic_api = Topic::get_client(&conf.main_api_endpoint);
            match topic_api
                .query(TopicQuery {
                    size: 10,
                    bookmark: None,
                    status: Some(TopicStatus::Ongoing),
                })
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    QueryResponse::default()
                }
            }
        })?;

        let finished_topics = use_server_future(move || async move {
            let topic_api = Topic::get_client(&conf.main_api_endpoint);
            match topic_api
                .query(TopicQuery {
                    size: 10,
                    bookmark: None,
                    status: Some(TopicStatus::Finished),
                })
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    QueryResponse::default()
                }
            }
        })?;

        let upcoming_topics = use_server_future(move || async move {
            let topic_api = Topic::get_client(&conf.main_api_endpoint);
            match topic_api
                .query(TopicQuery {
                    size: 10,
                    bookmark: None,
                    status: Some(TopicStatus::Scheduled),
                })
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list topics error: {:?}", e);
                    QueryResponse::default()
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

    pub fn ongoing_topics(&self) -> Vec<TopicSummary> {
        self.topics.with(|f| {
            tracing::debug!("main topic: {:?}", f);

            f.clone().unwrap_or_default().items
        })
    }

    pub fn finished_topics(&self) -> Vec<TopicSummary> {
        self.finished_topics.with(|f| {
            tracing::debug!("finished topic: {:?}", f);
            f.clone().unwrap_or_default().items
        })
    }

    pub fn upcoming_topics(&self) -> Vec<TopicSummary> {
        self.upcoming_topics.with(|f| {
            tracing::debug!("upcoming topic: {:?}", f);
            f.clone().unwrap_or_default().items
        })
    }
}
