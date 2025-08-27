use crate::Follower;
use crate::utils::users::extract_user_id;
use bdk::prelude::*;
use dto::{
    Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::{PgPool, Pool, Postgres},
};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct NetworkResponse {
    pub invitations: Vec<Follower>,
    pub suggestions: Vec<Follower>,
}

pub async fn list_networks_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
) -> Result<Json<NetworkResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    let invitations = get_invitations(pool.clone(), user_id).await?;
    let suggestions = get_suggestions(pool.clone(), user_id).await?;

    Ok(Json(NetworkResponse {
        invitations,
        suggestions,
    }))
}

pub async fn get_invitations(pool: Pool<Postgres>, user_id: i64) -> Result<Vec<Follower>> {
    let sql = r#"
        SELECT u.*
        FROM my_networks mn
        JOIN users u ON u.id = mn.follower_id
        WHERE mn.following_id = $1
          AND NOT EXISTS (
            SELECT 1
            FROM my_networks mf
            WHERE mf.follower_id = $1
              AND mf.following_id = u.id
          )
          AND NOT EXISTS (
            SELECT 1
            FROM connection_invitation_declines d
            WHERE d.user_id = $1
              AND d.decline_user_id = u.id
          )
        ORDER BY mn.created_at DESC
        LIMIT 50
    "#;

    let rows = dto::sqlx::query(sql)
        .bind(user_id)
        .map(Follower::from)
        .fetch_all(&pool)
        .await?;

    Ok(rows)
}

pub async fn get_suggestions(pool: Pool<Postgres>, user_id: i64) -> Result<Vec<Follower>> {
    let sql = r#"
        SELECT u.*
        FROM users u
        WHERE u.id <> $1
          AND NOT EXISTS (
              SELECT 1
              FROM my_networks mn
              WHERE mn.follower_id = $1
                AND mn.following_id = u.id
          )
          AND NOT EXISTS (
              SELECT 1
              FROM my_networks mn
              WHERE mn.follower_id = u.id
                AND mn.following_id = $1
          )
          AND NOT EXISTS (
              SELECT 1
              FROM user_suggestion_dismissals d
              WHERE d.user_id = $1
                AND d.dismissal_user_id = u.id
          )
        ORDER BY u.updated_at DESC, RANDOM()
        LIMIT 6
    "#;

    let rows = dto::sqlx::query(sql)
        .bind(user_id)
        .map(Follower::from)
        .fetch_all(&pool)
        .await?;

    Ok(rows)
}
