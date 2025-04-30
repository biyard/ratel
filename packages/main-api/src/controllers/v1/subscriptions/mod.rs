use crate::utils::users::extract_user_email;
use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::post,
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct SubscriptionController {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: SubscriptionRepository,
}

impl SubscriptionController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Subscription::get_repository(pool.clone());
        Self { pool, repo }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route(
                "/",
                post(Self::act_subscription).get(Self::list_subscriptions),
            )
            .with_state(self.clone())
    }

    pub async fn act_subscription(
        State(ctrl): State<SubscriptionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SubscriptionAction>,
    ) -> Result<Json<String>> {
        tracing::debug!("act_subscription {:?}", body);

        match body {
            SubscriptionAction::Subscribe(req) => {
                let _ = Json(ctrl.subscribe(req).await?);
                Ok(Json("ok".to_string()))
            }
            SubscriptionAction::Sponsor(_) => {
                ctrl.notify_slack(auth).await?;
                Ok(Json("ok".to_string()))
            }
        }
    }

    pub async fn list_subscriptions(
        State(ctrl): State<SubscriptionController>,
        Query(p): Query<SubscriptionParam>,
    ) -> Result<Json<QueryResponse<SubscriptionSummary>>> {
        tracing::debug!("list_subscriptions: {:?}", p);

        match p {
            SubscriptionParam::Query(q) => Ok(Json(ctrl.query(q).await?)),
        }
    }
}

impl SubscriptionController {
    async fn subscribe(&self, req: SubscriptionSubscribeRequest) -> Result<Subscription> {
        let subscription = self.repo.insert(req.email).await?;

        Ok(subscription)
    }

    async fn notify_slack(&self, auth: Option<Authorization>) -> Result<()> {
        let config = crate::config::get();
        let email = match extract_user_email(&self.pool, auth).await {
            Ok(email) => email,
            Err(e) => {
                tracing::error!("Failed to extract user email: {:?}", e);
                return Err(e);
            }
        };

        tracing::debug!("notify_slack: {:?}", email);

        let msg = format!(
            "Ratel: New sponsorship request from {}. Please check the email.",
            email
        );

        btracing::notify!(config.slack_webhook_url.to_string(), &msg);
        Ok(())
    }

    async fn query(&self, query: SubscriptionQuery) -> Result<QueryResponse<SubscriptionSummary>> {
        let mut total_count = 0;
        let items: Vec<SubscriptionSummary> = Subscription::query_builder()
            .limit(query.size())
            .page(query.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;
        Ok(QueryResponse { total_count, items })
    }
}
