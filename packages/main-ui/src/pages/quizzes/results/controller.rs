use bdk::prelude::{dioxus_popup::PopupService, *};
use dto::{PresidentialCandidate, QuizResult};

use crate::pages::landing::components::SignupPopup;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub result: Resource<(QuizResult, PresidentialCandidate)>,
    pub id: ReadOnlySignal<String>,
    pub popup: PopupService,
    #[cfg(feature = "web")]
    pub anonymous: crate::services::anonymouse_service::AnonymouseService,
    #[cfg(feature = "web")]
    pub user_service: crate::services::user_service::UserService,
    pub is_mine: Signal<bool>,
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

        #[allow(unused_mut)]
        let mut ctrl = Self {
            lang,
            result,
            id,
            popup: use_context(),
            #[cfg(feature = "web")]
            anonymous: use_context(),
            #[cfg(feature = "web")]
            user_service: use_context(),
            is_mine: use_signal(|| false),
        };

        #[cfg(feature = "web")]
        use_effect(move || {
            let anon = ctrl.anonymous.get_principal().eq(&ctrl.id());
            let m = anon && !ctrl.user_service.is_logined();
            ctrl.is_mine.set(m);
        });

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
}
