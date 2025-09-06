use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Error, FeedBookmarkUser, Result,
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
pub struct RemoveBookmarkRequest {
    #[schemars(description = "Feed ID")]
    pub feed_id: i64,
}

pub async fn remove_bookmark_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<RemoveBookmarkRequest>,
) -> Result<()> {
    let repo = FeedBookmarkUser::get_repository(pool.clone());

    let user_id = extract_user_id(&pool, auth).await?;

    let bookmark = FeedBookmarkUser::query_builder()
        .feed_id_equals(req.feed_id)
        .user_id_equals(user_id)
        .query()
        .map(FeedBookmarkUser::from)
        .fetch_optional(&pool.clone())
        .await?;

    if bookmark.is_none() {
        return Err(Error::NotFound);
    }

    let _ = repo.delete(bookmark.unwrap().id).await?;

    Ok(())
}
