use bdk::prelude::{by_axum::axum::extract::Path, *};
use by_axum::axum::{Json, extract::State, routing::get};
use dto::*;

#[derive(Clone, Debug)]
pub struct SponsorshipController {
    _pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct SlackNotifyPath {
    email: String,
}

impl SponsorshipController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { _pool: pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/:email", get(Self::notify_slack))
            .with_state(self.clone())
    }
}

impl SponsorshipController {
    async fn notify_slack(
        State(_ctrl): State<SponsorshipController>,
        Path(SlackNotifyPath { email }): Path<SlackNotifyPath>,
    ) -> Result<Json<()>> {
        tracing::debug!("notify_slack email: {:?}", email);
        let config = crate::config::get();
        let msg = format!(
            "Ratel: New sponsorship request from {}. Please check the email.",
            email
        );
        btracing::notify!(config.slack_webhook_url.to_string(), &msg);
        Ok(Json(()))
    }
}
