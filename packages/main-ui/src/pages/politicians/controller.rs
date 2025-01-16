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

    pub fn load_more(&mut self, lang: Language, bookmark: Option<String>) -> Result<(), RenderError> {
        let politician_api: PoliticianService = use_context();

        let current_politicians = self.politicians.clone();

        // self.politicians = use_server_future(move || async move {
        // // Fetch the next page of politicians using the provided bookmark
        //     match politician_api.list_politicians(
        //         20,  // Page size remains constant
        //         bookmark,  // Pass the bookmark for pagination
        //         Some(lang),  // Language preference
        //     ).await {
        //         Ok(new_politicians) => {
        //             // Get the current data
        //             let mut current_data = current_politicians.read().data.clone();

        //             // Append new politicians to the existing list
        //             current_data.extend(new_politicians.data.clone());

        //             // Create updated response with combined data and new bookmark
        //             CommonQueryResponse {
        //                 data: current_data,
        //                 bookmark: new_politicians.bookmark,
        //                 // Maintain other fields from the new response
        //                 ..new_politicians
        //             }
        //         },
        //         Err(e) => {
        //             // Log error and return current state on failure
        //             tracing::error!("load more politicians error: {:?}", e);
        //             current_politicians.read().clone()
        //         }
        //     }
        // })?;
    
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