use bdk::prelude::{by_axum::axum::extract::Path, *};
use by_axum::axum::{
    Json,
    extract::{Query, State},
    routing::{get, post},
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct BillPath {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct BillController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl BillController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::list_bills))
            .with_state(self.clone())
            .route("/:id", post(Self::get_file_link).with_state(self.clone()))
    }

    pub async fn list_bills(
        State(ctrl): State<BillController>,
        Query(p): Query<BillParam>,
    ) -> Result<Json<BillGetResponse>> {
        tracing::debug!("list_bills: {:?}", p);

        match p {
            BillParam::Query(q) => Ok(Json(BillGetResponse::Query(ctrl.query(q).await?))),
        }
    }

    pub async fn get_file_link(
        State(ctrl): State<BillController>,
        Path(BillPath { id }): Path<BillPath>,
        // Json(body): Json<>, // TODO: decide file type
    ) -> Result<String> {
        tracing::debug!("get_file_link: Bill ID: {:?}", id);
        let conf = crate::config::get();
        let bill = ctrl.get(id).await?;

        Ok(format!(
            "{}?bookId={}&type={}",
            conf.assembly_system_url,
            bill.book_id,
            "0" // 0: hwp 1: pdf
        ))
    }
}

impl BillController {
    async fn query(&self, query: BillQuery) -> Result<QueryResponse<BillSummary>> {
        let mut total_count = 0;
        let items: Vec<BillSummary> = BillSummary::query_builder()
            .limit(query.size())
            .page(query.page())
            .order_by_bill_no_desc()
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

    async fn get(&self, id: i64) -> Result<Bill> {
        let bill: Bill = Bill::query_builder()
            .id_equals(id)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;
        Ok(bill)
    }
}
