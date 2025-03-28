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
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<SubscriptionAction>,
    ) -> Result<Json<Subscription>> {
        tracing::debug!("act_subscription {:?}", body);

        match body {
            SubscriptionAction::Subscribe(req) => Ok(Json(ctrl.subscribe(req).await?)),
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
