use crate::utils::users::extract_user_id;
use bdk::prelude::*;
use dto::{
    Follower, Mynetwork, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
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
pub struct AcceptInvitationRequest {
    #[schemars(description = "Total Invitation IDs")]
    pub invitation_ids: Vec<i64>,
    #[schemars(description = "Followee ID")]
    pub followee_id: i64,
}

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
pub struct AcceptInvitationResponse {
    pub followee_id: i64,
    pub invitation: Option<Follower>,
}

pub async fn accept_invitation_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(body): Json<AcceptInvitationRequest>,
) -> Result<Json<AcceptInvitationResponse>> {
    let repo = Mynetwork::get_repository(pool.clone());
    let user_id = extract_user_id(&pool, auth).await?;
    let followee_id = body.followee_id;

    let _ = repo.insert_with_tx(&pool, user_id, followee_id).await?;

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
          AND u.id <> ALL($2::bigint[])
        ORDER BY mn.created_at DESC
        LIMIT 1
    "#;

    let mut excluded = body.invitation_ids.clone();
    excluded.push(user_id);
    excluded.push(followee_id);

    let next = dto::sqlx::query(sql)
        .bind(user_id)
        .bind(&excluded)
        .map(Follower::from)
        .fetch_optional(&pool)
        .await?;

    Ok(Json(AcceptInvitationResponse {
        followee_id,
        invitation: next,
    }))
}
