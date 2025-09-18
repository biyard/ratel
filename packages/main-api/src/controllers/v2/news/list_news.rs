use bdk::prelude::*;
use dto::{
    Error, Result,
    by_axum::axum::{Json, extract::{Query, State}},
    sqlx::PgPool,
    *,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, JsonSchema, aide::OperationIo)]
pub struct ListNewsQuery {
    /// Maximum number of items to return
    pub limit: Option<i64>,
}

/// GET /v2/news?limit=<n>
/// Returns latest news ordered by created_at desc
pub async fn list_news_handler(
    State(pool): State<PgPool>,
    Query(ListNewsQuery { limit }): Query<ListNewsQuery>,
) -> Result<Json<Vec<NewsSummary>>> {
    let limit = limit.unwrap_or(3).clamp(1, 100) as i32;

    let items: Vec<NewsSummary> = NewsSummary::query_builder()
        .limit(limit)
        .order_by_created_at_desc()
        .query()
        .map(NewsSummary::from)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error=?e, "Failed to query news");
            Error::ServerError("failed to query news".into())
        })?;

    Ok(Json(items))
}
