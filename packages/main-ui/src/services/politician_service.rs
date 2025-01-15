pub type Result<T> = std::result::Result<T, ServiceError>;

use dioxus::prelude::*;
use dioxus_translate::*;
use dto::{
    common_query_response::CommonQueryResponse, error::ServiceError, AssemblyMember,
    AssemblyMembersQuery,
};

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
    ) -> Result<CommonQueryResponse<AssemblyMember>> {
        // FIXME: provide full spec query
        let req = AssemblyMembersQuery {
            size: Some(size),
            bookmark: bookmark.map(|s| s.to_string()),
            lang,
            ..AssemblyMembersQuery::default()
        };

        let url = format!("{}/v1/assembly_members?{}", (self.endpoint)(), req);

        tracing::debug!("url: {}", url);
        let res: CommonQueryResponse<AssemblyMember> = match rest_api::get(&url).await {
            Ok(v) => v,
            Err(e) => match e {
                ServiceError::NotFound => {
                    return Ok(CommonQueryResponse::default());
                }
                e => {
                    return Err(e);
                }
            },
        };

        Ok(res)
    }
}
