use crate::services::user_service::{UserInfo, UserService};
use bdk::prelude::*;
#[allow(unused_imports)]
use dioxus_popup::PopupService;
use urlencoding::encode;
#[derive(Clone, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub user_info: UserInfo,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();

        let user_info = user_service.user_info();
        tracing::debug!("user info: {:?}", user_info);

        let ctrl = Self { lang, user_info };

        Ok(ctrl)
    }

    pub async fn notify_slack(&self) {
        let config = crate::config::get();
        let client = reqwest::Client::new();

        let email = self.user_info.email.clone().unwrap_or_default();

        let url = format!(
            "{}/v1/sponsorships/{}",
            config.main_api_endpoint,
            encode(&email)
        );

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    tracing::debug!("Slack notification sent successfully");
                } else {
                    tracing::error!("Failed to send Slack notification");
                }
            }
            Err(e) => {
                tracing::error!("Error sending Slack notification: {}", e);
            }
        }
    }

    #[allow(dead_code)]
    pub fn login_check(&self) {
        if !self.user_info.is_login() {}
    }
}
