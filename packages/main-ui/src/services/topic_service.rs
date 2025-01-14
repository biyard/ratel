pub type Result<T> = std::result::Result<T, ServiceError>;

use dioxus::prelude::*;
use dto::{
    common_query_response::CommonQueryResponse, error::ServiceError, TopicStatus, TopicSummery,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct TopicService {
    pub endpoint: Signal<String>,
}

impl TopicService {
    pub fn init() {
        let conf = crate::config::get();
        let srv = Self {
            endpoint: use_signal(|| conf.main_api_endpoint.clone()),
        };
        use_context_provider(|| srv);
    }

    pub async fn list_topics_by_status(
        &self,
        size: usize,
        bookmark: Option<&str>,
        status: Option<TopicStatus>,
    ) -> Result<CommonQueryResponse<TopicSummery>> {
        let client = reqwest::Client::builder().build()?;

        let mut url = format!("{}/v1/topics?size={size}", (self.endpoint)(),);

        if let Some(bookmark) = bookmark {
            url.push_str(&format!("&bookmark={}", bookmark));
        }

        if let Some(status) = status {
            url.push_str(&format!("&status={}", status));
        }

        tracing::debug!("url: {}", url);
        let res = client.request(reqwest::Method::GET, url).send().await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(res.json().await?)
        }
    }
}
