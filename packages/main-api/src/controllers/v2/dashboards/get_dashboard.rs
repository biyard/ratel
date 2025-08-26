use bdk::prelude::*;
use dto::{
    Feed, FeedSummary, Result, Space,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
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
pub struct GetDashboardResponse {
    pub top_spaces: Vec<Space>,
    pub matched_feeds: Vec<FeedSummary>,
    pub new_feeds: Vec<FeedSummary>,
}

pub async fn get_dashboard_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
) -> Result<Json<GetDashboardResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    //FIXME: fix by top spaces condition
    let spaces: Vec<Space> = Space::query_builder(user_id)
        .limit(3)
        .page(1)
        .order_by_created_at_desc()
        .query()
        .map(Space::from)
        .fetch_all(&pool)
        .await?;

    let mut top_spaces = vec![];

    for space in spaces.clone() {
        let feed_id = space.feed_id;

        let feed = Feed::query_builder(user_id)
            .id_equals(feed_id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await?;

        let mut s = space.clone();
        s.image_url = feed.url;
        s.likes = feed.likes;
        s.rewards = feed.rewards;
        s.number_of_comments = feed.comments;

        top_spaces.push(s);
    }

    tracing::debug!("top spaces: {:?}", top_spaces);

    //FIXME: fix matched feeds by using ai
    let matched_feeds: Vec<FeedSummary> = FeedSummary::query_builder(user_id)
        .limit(1)
        .page(1)
        .order_by_created_at_desc()
        .query()
        .map(FeedSummary::from)
        .fetch_all(&pool)
        .await?;

    tracing::debug!("matched feeds: {:?}", matched_feeds);

    let new_feeds: Vec<FeedSummary> = FeedSummary::query_builder(user_id)
        .limit(5)
        .page(1)
        .order_by_created_at_desc()
        .query()
        .map(FeedSummary::from)
        .fetch_all(&pool)
        .await?;

    tracing::debug!("new feeds: {:?}", new_feeds);

    Ok(Json(GetDashboardResponse {
        top_spaces,
        matched_feeds,
        new_feeds,
    }))
}
