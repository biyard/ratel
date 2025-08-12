use crate::tracing::error;
use dto::{
    Result,
    by_axum::axum::{Json, extract::State},
    sqlx::PgPool,
    *,
};

pub async fn list_industries(State(pool): State<PgPool>) -> Result<Json<Vec<Industry>>> {
    let industries = Industry::query_builder()
        .query()
        .map(Industry::from)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!("Failed to query industies {:?}", e);
            Error::NotFound
        })?;

    Ok(Json(industries))
}
