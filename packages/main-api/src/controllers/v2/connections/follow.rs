use bdk::prelude::*;
use dto::{
    Mynetwork, NotificationData, Result,
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
pub struct FollowRequest {
    #[schemars(description = "Followee IDs")]
    pub followee_ids: Vec<i64>,
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
pub struct FollowResponse {
    pub followee_ids: Vec<i64>,
}

pub async fn connection_follow_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(body): Json<FollowRequest>,
) -> Result<Json<FollowResponse>> {
    let repo = Mynetwork::get_repository(pool.clone());
    let user_id = extract_user_id(&pool, auth).await?;
    tracing::debug!("user id: {:?}", user_id);

    let followee_ids = body.followee_ids;

    let mut tx = pool.begin().await.unwrap();
    for followee_id in followee_ids.clone() {
        let _d = repo.insert_with_tx(&mut *tx, user_id, followee_id).await?;

        let notification_data = NotificationData::ConnectNetwork {
            requester_id: user_id,
            image_url: "".to_string(),
            description: "Someone has started following you".to_string(),
        };

        if let Err(e) =
            send_notification(&pool.clone(), &mut tx, followee_id, notification_data).await
        {
            tracing::error!(
                "Failed to send ConnectNetwork notification to user {}: {:?}",
                followee_id,
                e
            );
        }
    }
    tx.commit().await.unwrap();

    Ok(Json(FollowResponse { followee_ids }))
}
