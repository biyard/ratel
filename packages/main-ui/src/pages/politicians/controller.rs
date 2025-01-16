use dioxus_aws::prelude::*;
use dioxus_translate::Language;
use dto::{common_query_response::CommonQueryResponse, AssemblyMember};
use crate::services::politician_service::PoliticianService;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub politicians: Resource<CommonQueryResponse<AssemblyMember>>,
}

impl Controller {
    pub fn new(lang: Language) -> Result<Self, RenderError> {
        let politician_api: PoliticianService = use_context();

        let politicians = use_server_future(move || async move {
            match politician_api.list_politicians(
                20,
                None,
                Some(lang),
            ).await {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list politicians error: {:?}", e);
                    CommonQueryResponse::<AssemblyMember>::default()
                }
            }
        })?;

        let ctrl = Self { politicians };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn _load_more(&mut self, _lang: Language, _bookmark: Option<String>) -> Result<(), RenderError> {
        let _politician_api: PoliticianService = use_context();

        // TODO: how can i update the Resource<>?
    
        Ok(())
    }

    pub fn politicians(&self) -> Vec<AssemblyMember> {
        self.politicians.with(|f| {
            // tracing::debug!("politicians: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                Vec::new()
            }
        })
    }
}