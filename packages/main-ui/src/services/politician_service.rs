use dioxus::prelude::*;
use dto::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct PoliticianService {
    pub endpoint: Signal<String>,
}

impl PoliticianService {
    pub fn init() {
        let conf = crate::config::get();
        let srv = Self {
            endpoint: use_signal(|| conf.main_api_endpoint.clone()),
        };
        use_context_provider(|| srv);
    }

    pub async fn list_politicians(
        &self,
        size: usize,
    ) -> Result<CommonQueryResponse<AssemblyMember>> {
        let client = reqwest::Client::builder().build()?;

        let url = format!("{}/v1/politicians?size={}", (self.endpoint)(), size);

        tracing::debug!("url: {}", url);
        let res = client.request(reqwest::Method::GET, url).send().await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(res.json().await?)
        }
    }
}