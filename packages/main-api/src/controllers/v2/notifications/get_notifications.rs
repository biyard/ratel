use bdk::prelude::*;
use dto::{
    Follower, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
    *,
};
use std::collections::{HashMap, HashSet};

use crate::utils::users::extract_user_id;

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
pub struct FollowerResponse {
    pub created_at: i64,
    pub follower: Follower,
    pub is_following: bool,
    pub is_rejecting: bool,
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
pub struct NotificationResponse {
    pub networks: Vec<FollowerResponse>,
}

pub async fn get_notifications_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
) -> Result<Json<NotificationResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    let notifications = Notification::query_builder()
        .user_id_equals(user_id)
        .notification_type_equals(dto::NotificationType::ConnectNetwork)
        .read_is_false()
        .query()
        .map(Notification::from)
        .fetch_all(&pool)
        .await?;

    let mut requester_ids: Vec<i64> = Vec::new();
    let mut created_at_map: HashMap<i64, i64> = HashMap::new();
    for n in &notifications {
        if let dto::NotificationData::ConnectNetwork { requester_id, .. } = n.metadata {
            requester_ids.push(requester_id);
            created_at_map.insert(requester_id, n.created_at);
        }
    }
    if requester_ids.is_empty() {
        return Ok(Json(NotificationResponse { networks: vec![] }));
    }

    let followers: Vec<Follower> =
        dto::sqlx::query(r#"SELECT u.* FROM users u WHERE u.id = ANY($1::bigint[])"#)
            .bind(&requester_ids)
            .map(Follower::from)
            .fetch_all(&pool)
            .await?;

    let mut follower_map: HashMap<i64, Follower> = HashMap::new();
    for f in followers {
        follower_map.insert(f.id, f);
    }

    let following_ids: HashSet<i64> = dto::sqlx::query_scalar(
        r#"
        SELECT following_id
        FROM my_networks
        WHERE follower_id = $1
          AND following_id = ANY($2::bigint[])
        "#,
    )
    .bind(user_id)
    .bind(&requester_ids)
    .fetch_all(&pool)
    .await?
    .into_iter()
    .collect();

    let rejecting_ids: HashSet<i64> = dto::sqlx::query_scalar(
        r#"
        SELECT decline_user_id
        FROM connection_invitation_declines
        WHERE user_id = $1
          AND decline_user_id = ANY($2::bigint[])
        "#,
    )
    .bind(user_id)
    .bind(&requester_ids)
    .fetch_all(&pool)
    .await?
    .into_iter()
    .collect();

    let mut networks: Vec<FollowerResponse> = Vec::new();
    for id in requester_ids {
        if let Some(f) = follower_map.get(&id) {
            let created_at = *created_at_map.get(&id).unwrap_or(&0);
            let is_following = following_ids.contains(&id);
            let is_rejecting = rejecting_ids.contains(&id);
            networks.push(FollowerResponse {
                created_at,
                follower: f.clone(),
                is_following,
                is_rejecting,
            });
        }
    }

    Ok(Json(NotificationResponse { networks }))
}
