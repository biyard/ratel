use crate::AppState;
use crate::Permissions;
use crate::controllers::v3::spaces::{SpacePostCommentPath, SpacePostCommentPathParam};
use crate::features::spaces::boards::dto::space_post_comment_response::SpacePostCommentResponse;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::features::spaces::boards::models::space_post_comment_like::SpacePostCommentLike;
use crate::models::feed::PostComment;
use crate::models::user::User;
use crate::types::{ListItemsQuery, ListItemsResponse, Partition, TeamGroupPermission};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

pub async fn list_space_comments_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePostCommentPathParam {
        space_pk,
        space_post_pk,
        space_post_comment_sk,
    }): SpacePostCommentPath,
    Query(_query): ListItemsQuery,
) -> Result<Json<ListItemsResponse<SpacePostCommentResponse>>, crate::Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(crate::Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(crate::Error::NoPermission);
    }

    let (comments, bookmark) =
        SpacePostComment::list_by_comment(&dynamo.client, space_post_pk, space_post_comment_sk)
            .await?;

    let mut like_keys = Vec::with_capacity(comments.len());
    for c in &comments {
        like_keys.push(c.like_keys(&user.pk));
    }

    let likes = SpacePostCommentLike::batch_get(&dynamo.client, like_keys).await?;

    let items: Vec<SpacePostCommentResponse> = comments
        .into_iter()
        .map(|comment| {
            let liked = likes.iter().any(|like| like == &comment);
            let mut resp: SpacePostCommentResponse = comment.into();
            resp.liked = liked;
            resp
        })
        .collect();

    let resp: ListItemsResponse<SpacePostCommentResponse> = (items, bookmark).into();
    Ok(Json(resp))
}
