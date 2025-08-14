use dto::{
    Follower, Result, UserType,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Query, State},
        },
    },
    sqlx::{self, PgPool},
};
use serde::{Deserialize, Serialize};

use crate::utils::users::extract_user_id;

#[derive(Debug, Serialize)]
pub struct NetworkResponse {
    pub suggested_teams: Vec<Follower>,
    pub suggested_users: Vec<Follower>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub keyword: Option<String>,
}

pub async fn list_networks_by_keyword_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(SearchQuery { keyword }): Query<SearchQuery>,
) -> Result<Json<NetworkResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    let kw_like = keyword
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| format!("%{}%", s));

    let suggested_teams_sql = r#"
        SELECT u.*
        FROM users u
        WHERE u.id != $1
          AND u.user_type = $2
          AND ($3::text IS NULL OR u.username ILIKE $3 OR u.nickname ILIKE $3)
        ORDER BY u.updated_at DESC
        LIMIT 3
    "#;

    let suggested_users_sql = r#"
        SELECT u.*
        FROM users u
        WHERE u.id != $1
          AND u.user_type = $2
          AND ($3::text IS NULL OR u.username ILIKE $3 OR u.nickname ILIKE $3)
        ORDER BY u.updated_at DESC
        LIMIT 5
    "#;

    let suggested_teams = sqlx::query(suggested_teams_sql)
        .bind(user_id)
        .bind(UserType::Team as i32)
        .bind(kw_like.as_deref())
        .map(Follower::from)
        .fetch_all(&pool)
        .await?;

    let suggested_users = sqlx::query(suggested_users_sql)
        .bind(user_id)
        .bind(UserType::Individual as i32)
        .bind(kw_like.as_deref())
        .map(Follower::from)
        .fetch_all(&pool)
        .await?;

    Ok(Json(NetworkResponse {
        suggested_teams,
        suggested_users,
    }))
}
