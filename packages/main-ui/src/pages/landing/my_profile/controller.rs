use bdk::prelude::*;

use crate::services::user_service::UserService;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub name: Signal<String>,
    pub email: ReadOnlySignal<String>,
    pub profile_url: Signal<String>,
    pub aggree_getting_info: Signal<bool>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();
        let user = user_service.user_info();

        let ctrl = Self {
            lang,
            profile_url: use_signal(move || user.profile_url.clone().unwrap_or_default()),
            name: use_signal(move || user.nickname.clone().unwrap_or_default()),
            email: use_signal(move || user.email.clone().unwrap_or_default()).into(),
            aggree_getting_info: use_signal(|| false),
        };

        Ok(ctrl)
    }

    pub fn upgrade_membership(&self) {
        tracing::debug!("Upgrade membership");
    }
}
