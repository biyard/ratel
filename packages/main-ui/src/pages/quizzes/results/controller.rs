use bdk::prelude::{dioxus_popup::PopupService, *};
use dto::{PresidentialCandidate, QuizResult};

use crate::{pages::landing::components::SignupPopup, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub result: Resource<(QuizResult, PresidentialCandidate)>,
    pub id: ReadOnlySignal<String>,
    pub popup: PopupService,
    #[cfg(feature = "web")]
    pub anonymous: crate::services::anonymouse_service::AnonymouseService,
    pub user_service: UserService,
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

        let ctrl = Self {
            lang,
            result,
            id,
            popup: use_context(),
            #[cfg(feature = "web")]
            anonymous: use_context(),
            user_service: use_context(),
        };

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

    pub fn sign_up(&mut self) {
        self.popup.open(rsx! {
            SignupPopup { lang: self.lang }
        });
    }

    pub fn is_mine(&self) -> bool {
        #[allow(unused_variables)]
        let anon = false;
        #[cfg(feature = "web")]
        let anon = self.anonymous.get_principal() == self.id();

        anon && !self.user_service.is_logined()
    }
}
