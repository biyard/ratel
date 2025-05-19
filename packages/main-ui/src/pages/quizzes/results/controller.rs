use bdk::prelude::*;
use dto::{PresidentialCandidate, QuizResult};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub result: Resource<(QuizResult, PresidentialCandidate)>,
    pub id: ReadOnlySignal<String>,
}

impl Controller {
    pub fn new(
        lang: Language,
        id: ReadOnlySignal<String>,
    ) -> std::result::Result<Self, RenderError> {
        let result = use_server_future(move || {
            let principal = id();
            async move {
                let result = QuizResult::get_client(crate::config::get().main_api_endpoint)
                    .get_result(principal)
                    .await
                    .unwrap_or_default();

                let candidate_id = result.most_supportive_candidate();
                let candidate =
                    PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
                        .get(candidate_id)
                        .await
                        .unwrap_or_default();

                (result, candidate)
            }
        })?;

        let ctrl = Self { lang, result, id };

        Ok(ctrl)
    }

    pub fn location(&self) -> String {
        let conf = crate::config::get();
        let url = format!(
            "{}%3A%2F%2F{}%2Fquizzes%2Fresults%2F{}",
            if conf.env == "local" { "http" } else { "https" },
            conf.domain,
            self.id()
        );

        url
    }
}
