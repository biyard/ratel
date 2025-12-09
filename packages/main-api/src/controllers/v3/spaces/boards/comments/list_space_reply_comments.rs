use std::collections::HashSet;

use crate::features::report::ContentReport;
use crate::features::spaces::boards::dto::space_post_comment_response::SpacePostCommentResponse;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::features::spaces::boards::models::space_post_comment::SpacePostCommentQueryOption;
use crate::features::spaces::boards::models::space_post_comment_like::SpacePostCommentLike;
use crate::spaces::SpacePostCommentPath;
use crate::spaces::SpacePostCommentPathParam;
use crate::*;

pub async fn list_space_reply_comments_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePostCommentPathParam {
        space_pk,
        space_post_pk,
        space_post_comment_sk,
    }): SpacePostCommentPath,
    Query(query): ListItemsQuery,
) -> Result<Json<ListItemsResponse<SpacePostCommentResponse>>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(crate::Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(crate::Error::NoPermission);
    }

    let mut opt = SpacePostCommentQueryOption::builder()
        .limit(10)
        .scan_index_forward(false);

    if let Some(bookmark) = query.bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (comments, bookmark) =
        SpacePostComment::list_by_comment(&dynamo.client, space_post_comment_sk, opt).await?;

    let mut like_keys = Vec::with_capacity(comments.len());
    for c in &comments {
        like_keys.push(c.like_keys(&user.pk));
    }

    let likes = SpacePostCommentLike::batch_get(&dynamo.client, like_keys).await?;

    let reported_comment_ids: HashSet<String> =
        ContentReport::reported_comment_ids_for_post_by_user(
            &dynamo.client,
            &space_post_pk,
            &user.pk,
        )
        .await?;

    let items: Vec<SpacePostCommentResponse> = comments
        .into_iter()
        .map(|comment| {
            let liked = likes.iter().any(|like| like == &comment);
            let sk_str = comment.sk.to_string();
            let reported = reported_comment_ids.contains(&sk_str);

            let mut resp: SpacePostCommentResponse = comment.into();
            resp.liked = liked;
            resp.is_report = reported;
            resp
        })
        .collect();

    let resp: ListItemsResponse<SpacePostCommentResponse> = (items, bookmark).into();
    Ok(Json(resp))
}
