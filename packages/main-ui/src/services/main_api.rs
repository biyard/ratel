pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use dioxus::prelude::*;
use dto::{common_query_response::CommonQueryResponse, Topic, TopicStatus};

#[derive(Debug, Clone, Copy, Default)]
pub struct MainApi {
    pub endpoint: Signal<String>,
}

impl MainApi {
    pub fn init() {
        let conf = crate::config::get();
        let srv = Self {
            endpoint: use_signal(|| conf.topic_api_endpoint.clone()),
        };
        use_context_provider(|| srv);
    }

    pub async fn list_topics(
        &self,
        size: usize,
        bookmark: Option<&str>,
        status: Option<TopicStatus>,
    ) -> Result<CommonQueryResponse<Topic>> {
        let client = reqwest::Client::builder().build()?;

        let mut url = format!("{}/v1/topics?size={size}", (self.endpoint)(),);

        if let Some(bookmark) = bookmark {
            url.push_str(&format!("&bookmark={}", bookmark));
        }

        if let Some(status) = status {
            url.push_str(&format!("&status={}", status));
        }

        tracing::debug!("url: {}", url);
        let request = client.request(reqwest::Method::GET, url);

        Ok(request.send().await?.json().await?)
    }
}
