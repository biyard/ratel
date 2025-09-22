use bdk::prelude::*;
use dto::{
    Error, Result,
    by_axum::axum::{Json, extract::{Query, State}},
    sqlx::{self, PgPool, Row},
    *,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, JsonSchema, aide::OperationIo)]
pub struct ListNewsQuery {
    /// Maximum number of items to return
    pub limit: Option<i32>,
    /// Page number (1-based)
    pub page: Option<i32>,
}


pub async fn list_news_handler(
    State(pool): State<PgPool>,
    Query(ListNewsQuery { limit, page }): Query<ListNewsQuery>,
) -> Result<Json<Vec<News>>> {
    let limit = limit.unwrap_or(10).clamp(1, 100);
    let page = page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

   
    let table_names: Vec<String> = sqlx::query_scalar(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();
    
    tracing::info!(tables = ?table_names, "Available tables in database");

    let table_name = "news";
    
    if !table_names.iter().any(|t| t.eq_ignore_ascii_case(table_name)) {
        return Err(Error::ServerError(format!(
            "News table not found. Available tables: {:?}",
            table_names
        )));
    }

   
    let query = format!(
        r#"
        SELECT 
            id,
            created_at,
            updated_at,
            title,
            html_content,
            user_id
        FROM {}
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        table_name
    );

    let rows = sqlx::query(&query)
        .bind(limit as i32)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to execute news query");
            Error::ServerError("Failed to fetch news".into())
        })?;

    let items = rows
        .into_iter()
        .filter_map(|row| {
            Some(News {
                id: row.try_get::<i64, _>("id").ok()?,
                created_at: row.try_get::<i64, _>("created_at").ok()?,
                updated_at: row.try_get::<i64, _>("updated_at").ok()?,
                title: row.try_get::<String, _>("title").ok()?,
                html_content: row.try_get::<String, _>("html_content").ok()?,
                user_id: row.try_get::<i64, _>("user_id").ok()?,
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(items))
}