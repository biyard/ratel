pub type Result<T> = std::result::Result<T, ServiceError>;

use dioxus::prelude::*;
use dioxus_translate::*;
use dto::{common_query_response::CommonQueryResponse, error::ServiceError, AssemblyMember};
use crate::utils::rest_api;

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
        bookmark: Option<&str>,
        lang: Option<Language>,
    ) -> Result<Vec<AssemblyMember>> {
        let mut url = format!("{}/v1/assembly_members?size={size}", (self.endpoint)(),);

        if let Some(bookmark) = bookmark {
            url.push_str(&format!("&bookmark={}", bookmark));
        }

        if let Some(lang) = lang {
            url.push_str(&format!("&lang={}", lang));
        }

        tracing::debug!("url: {}", url);
        let res: Vec<AssemblyMember> = match rest_api::get(&url).await {
            Ok(v) => v,
            Err(e) => match e {
                ServiceError::NotFound => {
                    return Ok(vec![]);
                }
                e => {
                    return Err(e);
                }
            },
        };

        Ok(res)
    }
}