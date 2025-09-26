use bdk::prelude::*;

use dto::{
    ArtworkMetadata, FeedType, File, GroupPermission, Post, PostRepositoryUpdateRequest,
    RatelResource, Result, UrlType, aide,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Path, State},
        },
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::{security::check_perm, utils::users::extract_user};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct UpdatePostParams {
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct UpdatePostRequest {
    pub team_id: Option<i64>,
    pub feed_type: Option<FeedType>,
    pub industry_id: Option<i64>,
    pub title: Option<String>,
    pub html_contents: Option<String>,
    pub url: Option<String>,
    pub url_type: Option<UrlType>,
    pub artwork_metadata: Option<ArtworkMetadata>,
    pub files: Option<Vec<File>>,
    pub rewards: Option<i64>,
}

pub async fn update_post_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Path(params): Path<UpdatePostParams>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<Post>> {
    /*
    Permission Check

    When team_id is provided, checking user has permission to edit posts in that team.
     */
    let (user, authorize_user_id) = if let Some(team_id) = req.team_id {
        let user = check_perm(
            &pool,
            auth,
            RatelResource::Post { team_id: team_id },
            GroupPermission::WritePosts,
        )
        .await?;
        (user, team_id)
    } else {
        let user = extract_user(&pool, auth).await?;
        let user_id = user.id;
        (user, user_id)
    };
    let post = Post::query_builder(user.id)
        .id_equals(params.id)
        .query()
        .map(Post::from)
        .fetch_one(&pool)
        .await?;

    let author = post.author.get(0).cloned().unwrap_or_default();
    if author.id != authorize_user_id {
        return Err(dto::Error::Unauthorized);
    }

    let repo = Post::get_repository(pool.clone());
    let res = repo
        .update(
            post.id,
            PostRepositoryUpdateRequest {
                feed_type: req.feed_type,
                industry_id: req.industry_id,
                title: req.title,
                html_contents: req.html_contents,
                url: req.url,
                url_type: req.url_type,
                files: req.files,
                artwork_metadata: req.artwork_metadata,
                rewards: req.rewards,
                user_id: None,
                parent_id: None,
                quote_feed_id: None,
                status: None,
            },
        )
        .await?;

    Ok(Json(res))
}
