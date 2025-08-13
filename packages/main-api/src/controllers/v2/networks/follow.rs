use dto::{
    Mynetwork, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::utils::users::extract_user_id;

#[derive(Debug, Deserialize)]
pub struct FollowRequest {
    pub followee_ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct FollowResponse {
    pub followee_ids: Vec<i64>,
}

pub async fn follow_handler(
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
    }
    tx.commit().await.unwrap();

    Ok(Json(FollowResponse { followee_ids }))
}
