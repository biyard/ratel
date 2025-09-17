use bdk::prelude::*;

use dto::{
    FeedStatus, FeedType, GroupPermission, Post, RatelResource, Result, Space, aide,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Query, State},
        },
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::{
    security::check_perm,
    utils::{
        space_visibility::{ViewerCtx, scope_space_opt_to_vec},
        users::extract_user,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct ListPostsQueryParams {
    pub size: Option<i32>,
    pub page: Option<i32>,
    pub status: Option<FeedStatus>,
    pub user_id: Option<i64>,
}

pub async fn list_posts_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(params): Query<ListPostsQueryParams>,
) -> Result<Json<Vec<Post>>> {
    let size = params.size.unwrap_or(10);
    let page = params.page.unwrap_or(1);

    let user = extract_user(&pool, auth.clone()).await.unwrap_or_default();
    let user_id = user.id;
    let teams = user.teams;

    if let Some(query_user_id) = params.user_id {
        tracing::debug!(
            "Checking permissions for user_id: {} {}",
            query_user_id,
            user_id
        );
        if query_user_id != user_id {
            check_perm(
                &pool,
                auth,
                RatelResource::Post {
                    team_id: query_user_id,
                },
                GroupPermission::ReadPosts,
            )
            .await?;
        }
    };

    let builder = Post::query_builder(user_id);
    let builder = if let Some(status) = params.status {
        builder.status_equals(status)
    } else {
        builder
    };

    let builder = if let Some(user_id) = params.user_id {
        if user_id != 0 {
            builder.user_id_equals(user_id)
        } else {
            builder
        }
    } else {
        builder
    };

    let mut posts: Vec<Post> = builder
        .limit(size)
        .page(page)
        .feed_type_between(FeedType::Artwork, FeedType::Post)
        .order_by_created_at_desc()
        .query()
        .map(Post::from)
        .fetch_all(&pool)
        .await?;

    //FIXME: Currently, `Post::query_builder(user_id).space_builder(Space::query_builder()) ` does not work properly.
    // So, we need to fetch Space separately.
    // this bug should be fixed in the future.
    // Same issue with `get_post_handler`
    let viewer_ctx = ViewerCtx {
        user_id,
        team_ids: teams.iter().map(|t| t.id).collect(),
    };
    for post in &mut posts {
        let space = Space::query_builder(user_id)
            .feed_id_equals(post.id)
            .query()
            .map(Space::from)
            .fetch_optional(&pool)
            .await?;

        post.space = scope_space_opt_to_vec(space, &viewer_ctx);
    }

    // let posts = posts
    //     .into_iter()
    //     .map(|mut post| {
    //         let space = post.space.get(0);
    //         if let Some(space) = space {
    //             // Check SpaceStatus::Draft and author is user or author is in user's teams
    //             if space.status == SpaceStatus::Draft
    //                 && space.author.get(0).is_some_and(|author| {
    //                     author.id == user_id && teams.iter().any(|t| t.id == author.id)
    //                 })
    //             {
    //                 post.space = vec![];
    //             }
    //         }

    //         post
    //     })
    //     .collect::<Vec<Post>>();
    Ok(Json(posts))
}
