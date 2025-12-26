use crate::services::biyard::ProjectPointTransactionResponse;
use crate::utils::time::current_month;
use crate::*;
use axum::{Json, extract::State};
use bdk::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ListTransactionsQuery {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub bookmark: Option<String>,
    #[serde(default)]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ListTransactionsResponse {
    pub items: Vec<ProjectPointTransactionResponse>,
    pub bookmark: Option<String>,
}

pub async fn list_transactions_handler(
    State(AppState { biyard, .. }): State<AppState>,
    Query(query): Query<ListTransactionsQuery>,
) -> Result<Json<ListTransactionsResponse>> {
    let date = query.date.unwrap_or_else(current_month);

    let result = biyard
        .get_all_transactions(Some(date), query.bookmark, query.limit)
        .await?;

    Ok(Json(ListTransactionsResponse {
        items: result.items,
        bookmark: result.bookmark,
    }))
}
