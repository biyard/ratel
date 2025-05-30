use bdk::prelude::*;
use dto::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub keyword: Signal<String>,
    pub candidates: Resource<Vec<PresidentialCandidateSummary>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let keyword = use_signal(|| String::new());

        let candidates = use_server_future(move || {
            let keyword = keyword();

            async move {
                let _ = keyword;
                let res = PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
                    .query(PresidentialCandidateQuery::new(20))
                    .await
                    .unwrap_or_default();
                res.items
            }
        })?;

        let ctrl = Self {
            lang,
            keyword,
            candidates,
        };

        Ok(ctrl)
    }
}
