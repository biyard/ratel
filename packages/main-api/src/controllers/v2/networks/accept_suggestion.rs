use bdk::prelude::*;
use dto::{
    Follower, Mynetwork, NotificationData, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};

use crate::utils::{notifications::send_notification, users::extract_user_id};

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
pub struct AcceptSuggestionRequest {
    #[schemars(description = "Total Suggestion IDs")]
    pub suggestion_ids: Vec<i64>,
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
pub struct AcceptSuggestionResponse {
    pub suggestion: Follower,
}

pub async fn accept_suggestion_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(body): Json<AcceptSuggestionRequest>,
) -> Result<Json<AcceptSuggestionResponse>> {
    let repo = Mynetwork::get_repository(pool.clone());
    let user_id = extract_user_id(&pool, auth).await?;
    let followee_id = body.followee_id;

    let mut tx = pool.begin().await?;

    let _ = repo.insert_with_tx(&mut *tx, user_id, followee_id).await?;

    let mut excluded_ids = body.suggestion_ids.clone();
    excluded_ids.push(followee_id);
    excluded_ids.push(user_id);

    let sql = r#"
        SELECT u.*
        FROM users u
        WHERE u.id <> $1
          AND NOT EXISTS (
              SELECT 1 FROM my_networks mn
              WHERE mn.follower_id = $1
                AND mn.following_id = u.id
          )
          AND NOT EXISTS (
              SELECT 1 FROM user_suggestion_dismissals d
              WHERE d.user_id = $1
                AND d.dismissal_user_id = u.id
          )
          AND NOT EXISTS (
              SELECT 1 FROM connection_invitation_declines cd
              WHERE cd.user_id = $1
                AND cd.decline_user_id = u.id
          )
          AND u.id <> ALL($2::bigint[])
        ORDER BY u.updated_at DESC, RANDOM()
        LIMIT 1
    "#;

    let next: Option<Follower> = dto::sqlx::query(sql)
        .bind(user_id)
        .bind(&excluded_ids)
        .map(Follower::from)
        .fetch_optional(&pool)
        .await?;

    let notification_data = NotificationData::ConnectNetwork {
        requester_id: user_id,
        image_url: "".to_string(),
        description: "Someone has started following you".to_string(),
    };

    if let Err(e) = send_notification(&pool.clone(), &mut tx, followee_id, notification_data).await
    {
        tracing::error!(
            "Failed to send ConnectNetwork notification to user {}: {:?}",
            followee_id,
            e
        );
    }

    let suggestion = next.ok_or(dto::Error::NotFound)?;

    tx.commit().await?;

    Ok(Json(AcceptSuggestionResponse { suggestion }))
}
