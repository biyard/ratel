use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    FeedBookmarkUser, Result,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

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
pub struct AddBookmarkRequest {
    #[schemars(description = "Feed ID")]
    pub feed_id: i64,
}

pub async fn add_bookmark_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<AddBookmarkRequest>,
) -> Result<()> {
    let repo = FeedBookmarkUser::get_repository(pool.clone());

    let user_id = extract_user_id(&pool, auth).await?;

    let _ = repo.insert(req.feed_id, user_id).await?;

    Ok(())
}
