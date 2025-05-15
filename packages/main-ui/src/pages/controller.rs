use bdk::prelude::*;

use crate::{route::Route, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();
        let nav = use_navigator();
        use_effect(move || {
            tracing::debug!("Checking login status {}", user_service.loggedin());
            if !user_service.loggedin() {
                nav.replace(Route::LandingPage {});
            }
        });

        let ctrl = Self { lang };

        Ok(ctrl)
    }
}
