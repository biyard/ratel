use bdk::prelude::*;
use dto::Error;
use dto::{
    Feed, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use serde_json::json;

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
pub struct GetBookmarksResponse {
    pub bookmarked_feeds: Vec<Feed>,
}

pub async fn get_bookmarks_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
) -> Result<Json<GetBookmarksResponse>> {
    let user_id = extract_user_id(&pool, auth).await?;

    let row: Option<(serde_json::Value,)> = sqlx::query_as(
        r#"
        WITH src AS (
          SELECT DISTINCT ON (f.id)
            f.id                                     ::bigint AS id,
            COALESCE(f.created_at, 0)               ::bigint AS created_at,
            COALESCE(f.updated_at, 0)               ::bigint AS updated_at,
            COALESCE(f.feed_type, 1)                ::int    AS feed_type,
            COALESCE(f.user_id, 0)                  ::bigint AS user_id,
            f.parent_id                                       AS parent_id,
            f.quote_feed_id                                    AS quote_feed_id,
            f.title                                           AS title,
            f.html_contents                                    AS html_contents,
            f.url                                             AS url,
            COALESCE(f.url_type, 0)                ::int      AS url_type,
            COALESCE(f.rewards, 0)                 ::bigint   AS rewards,
            COALESCE(f.status, 2)                  ::int      AS status,
            COALESCE(f.files, '[]'::jsonb)                   AS files,
            COALESCE(f.industry_id, 0)             ::bigint   AS industry_id,

            (SELECT COUNT(*) FROM feed_users fu WHERE fu.feed_id = f.id)  ::bigint AS likes,
            (SELECT EXISTS(SELECT 1 FROM feed_users fu
                           WHERE fu.feed_id = f.id AND fu.user_id = $1))           AS is_liked,
            (SELECT COUNT(*) FROM feeds c WHERE c.parent_id = f.id)       ::bigint AS comments,
            (SELECT COUNT(*) FROM feed_shares fs WHERE fs.feed_id = f.id) ::bigint AS shares,
            (SELECT EXISTS(SELECT 1 FROM feed_bookmark_users fb
                           WHERE fb.feed_id = f.id AND fb.user_id = $1))           AS is_bookmarked,
            (SELECT EXISTS(SELECT 1 FROM onboards o WHERE o.meta_id = f.id))       AS onboard,

            '[]'::jsonb AS spaces,
            '[]'::jsonb AS industry,
            '[]'::jsonb AS comment_list,

            (
              SELECT COALESCE(jsonb_agg(to_jsonb(a)), '[]'::jsonb)
              FROM (
                SELECT
                  u.id                     ::bigint AS id,
                  COALESCE(u.created_at,0) ::bigint AS created_at,
                  COALESCE(u.updated_at,0) ::bigint AS updated_at,
                  u.nickname,
                  u.principal,
                  u.email,
                  COALESCE(u.profile_url,'')        AS profile_url,
                  COALESCE(u.user_type, 1) ::int    AS user_type,
                  COALESCE(u.username,'')           AS username
                FROM users u
                WHERE u.id = f.user_id
              ) a
            ) AS author
          FROM feeds f
          JOIN feed_bookmark_users j ON j.feed_id = f.id
          WHERE j.user_id = $1
          ORDER BY f.id DESC
        )
        SELECT COALESCE(jsonb_agg(to_jsonb(src)), '[]'::jsonb) AS items
        FROM (SELECT 1) d
        LEFT JOIN src ON TRUE
        "#,
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await?;

    let items_json = row.map(|(v,)| v).unwrap_or_else(|| json!([]));

    let bookmarked_feeds: Vec<Feed> = match serde_json::from_value(items_json.clone()) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("deserialize error: {e}; payload: {items_json}");
            return Err(Error::BadRequest);
        }
    };

    Ok(Json(GetBookmarksResponse { bookmarked_feeds }))
}
