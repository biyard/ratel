#![allow(warnings)]
use crate::{
    AppState, Error, Permissions,
    controllers::v3::spaces::{SpacePath, SpacePathParam, SpacePostPath, SpacePostPathParam},
    features::spaces::boards::{
        dto::{
            space_post_comment_response::SpacePostCommentResponse,
            space_post_response::SpacePostResponse,
        },
        models::{
            space_category::SpaceCategory,
            space_post::SpacePost,
            space_post_comment::{SpacePostComment, SpacePostCommentQueryOption},
            space_post_comment_like::SpacePostCommentLike,
        },
    },
    models::{SpaceCommon, feed::Post, team::Team, user::User},
    types::{EntityType, Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

pub async fn get_space_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
) -> Result<Json<SpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);
    let post = SpacePost::get(&dynamo.client, pk, Some(sk))
        .await?
        .ok_or(Error::PostNotFound)?;

    let mut bookmark = None::<String>;
    let mut comments = vec![];
    // FIXME: 여기서 모든 Comment 를 한번에 읽어올 필요가 없음. Pagenation 을 넣고, 별도 API 로 Comment 추가 로드 코드 분리
    loop {
        let mut option = SpacePostCommentQueryOption::builder()
            .sk(EntityType::SpacePostComment(String::default()).to_string())
            .limit(100);
        if let Some(b) = &bookmark {
            option = option.bookmark(b.clone());
        }
        let (responses, next_bookmark) =
            SpacePostComment::query(&dynamo.client, space_post_pk.clone(), option).await?;

        let comment_keys = responses
            .iter()
            .map(|r| r.like_keys(&user.pk))
            .collect::<Vec<_>>();
        let comment_likes = SpacePostCommentLike::batch_get(&dynamo.client, comment_keys).await?;
        for response in responses {
            let liked = comment_likes.iter().any(|like| like == &response);
            let mut c: SpacePostCommentResponse = response.into();
            c.liked = liked;
            comments.push(c.into());
        }

        match next_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    let mut post: SpacePostResponse = post.into();
    post.comments = comments;

    Ok(Json(post))
}
