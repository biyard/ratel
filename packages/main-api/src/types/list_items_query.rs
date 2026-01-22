use axum::extract::Query;
use bdk::prelude::*;

pub type ListItemsQuery = Query<Pagination>;
pub type PaginationQuery = Query<Pagination>;

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct Pagination {
    #[schemars(description = "Bookmark to start from")]
    pub bookmark: Option<String>,
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct MonthQuery {
    #[schemars(description = "Month in YYYY-MM format (defaults to current month)")]
    pub month: Option<String>,
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct TransactionsQuery {
    #[serde(flatten)]
    pub pagination: Pagination,
    #[schemars(description = "Month in YYYY-MM format (defaults to current month)")]
    pub month: Option<String>,
}
