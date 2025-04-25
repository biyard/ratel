use bdk::prelude::*;
use dto::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub candidates: Resource<Vec<PresidentialCandidateSummary>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let candidates = use_server_future(move || async move {
            match PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
                .query(PresidentialCandidateQuery::new(20))
                .await
            {
                Ok(members) => members.items,
                _ => Default::default(),
            }
        })?;

        let ctrl = Self { lang, candidates };

        Ok(ctrl)
    }
}
