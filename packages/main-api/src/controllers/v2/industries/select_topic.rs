use dto::{
    Result, UserIndustry,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::utils::users::extract_user_id;

#[derive(Debug, Deserialize)]
pub struct SelectTopicsRequest {
    pub topics: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct SelectTopicsResponse {
    pub topics: Vec<i64>,
}

pub async fn select_topics_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(body): Json<SelectTopicsRequest>,
) -> Result<Json<SelectTopicsResponse>> {
    let repo = UserIndustry::get_repository(pool.clone());

    tracing::debug!("select topics: {:?}", body);
    let user_id = extract_user_id(&pool, auth).await?;
    tracing::debug!("user id: {:?}", user_id);

    let industry_ids = body.topics;

    let mut tx = pool.begin().await.unwrap();

    for industry_id in industry_ids.clone() {
        let _d = repo.insert_with_tx(&mut *tx, user_id, industry_id).await?;
    }

    tx.commit().await.unwrap();

    Ok(Json(SelectTopicsResponse {
        topics: industry_ids,
    }))
}
