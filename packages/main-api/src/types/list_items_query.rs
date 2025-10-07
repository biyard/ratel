use axum::extract::Query;
use bdk::prelude::*;

pub type ListItemsQuery = Query<Pagination>;

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
