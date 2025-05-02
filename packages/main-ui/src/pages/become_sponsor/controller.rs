use super::components::sponsor_confirm_popup::SponsorConfirmPopup;
use crate::{
    route::Route,
    services::user_service::{UserInfo, UserService},
};
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dto::*;

#[derive(Clone, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub user_info: UserInfo,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();
        let user_info = user_service.user_info();
        tracing::debug!("user info: {:?}", user_info);

        let nav = use_navigator();
        if !user_info.is_login() {
            nav.push(Route::HomePage {});
        }

        let ctrl = Self {
            lang,
            user_info,
            nav,
        };

        Ok(ctrl)
    }

    pub async fn notify_slack(&self) {
        let client = Subscription::get_client(&crate::config::get().main_api_endpoint);
        let mut popup: PopupService = use_context();
        let email = match self.user_info.email.clone() {
            Some(email) => email,
            None => {
                tracing::error!("user email not found");
                self.nav.push(Route::HomePage {});
                return;
            }
        };

        // FIXME: notify slack error: Unknown("error decoding response body")
        match client.sponsor(email).await {
            Ok(_) => {
                popup
                    .open(rsx! {
                        SponsorConfirmPopup { lang: self.lang }
                    })
                    .with_id("sponsor_confirm_popup");
            }
            Err(e) => {
                tracing::error!("notify slack error: {:?}", e);
                popup
                    .open(rsx! {
                        SponsorConfirmPopup { lang: self.lang }
                    })
                    .with_id("sponsor_confirm_popup");
            }
        }
    }
}
