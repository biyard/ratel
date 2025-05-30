use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::get,
    },
};
use by_types::QueryResponse;
use dto::*;

#[derive(Clone, Debug)]
pub struct TotalController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl TotalController {
    async fn query(&self, _auth: Option<Authorization>) -> Result<QueryResponse<TotalInfoSummary>> {
        tracing::debug!("hello");
        let mut total_count: i64 = 0;
        let items: Vec<TotalInfoSummary> = TotalInfoSummary::query_builder()
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl TotalController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_total_infos))
            .with_state(self.clone()))
    }

    pub async fn get_total_infos(
        State(ctrl): State<TotalController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<TotalInfoParam>,
    ) -> Result<Json<TotalInfoGetResponse>> {
        tracing::debug!("list_totals {:?}", q);

        match q {
            TotalInfoParam::Query(_param) => {
                Ok(Json(TotalInfoGetResponse::Query(ctrl.query(auth).await?)))
            }
        }
    }
}
