use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::post},
};
use dto::*;

#[derive(Clone, Debug)]
pub struct SubscriptionController {
    repo: SubscriptionRepository,
}

impl SubscriptionController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Subscription::get_repository(pool.clone());
        Self { repo }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", post(Self::act_subscription))
            .with_state(self.clone())
    }

    pub async fn act_subscription(
        State(ctrl): State<SubscriptionController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<SubscriptionAction>,
    ) -> Result<Json<Subscription>> {
        tracing::debug!("act_subscription {:?}", body);

        match body {
            SubscriptionAction::Subscribe(req) => Ok(Json(ctrl.subscribe(req).await?)),
        }
    }
}

impl SubscriptionController {
    async fn subscribe(&self, req: SubscriptionSubscribeRequest) -> Result<Subscription> {
        let subscription = self.repo.insert(req.email).await?;

        Ok(subscription)
    }
}
