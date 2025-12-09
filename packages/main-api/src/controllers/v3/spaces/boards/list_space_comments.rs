use std::collections::HashSet;

use crate::features::report::ContentReport;
use crate::features::spaces::boards::dto::space_post_comment_response::SpacePostCommentResponse;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::features::spaces::boards::models::space_post_comment::SpacePostCommentQueryOption;
use crate::features::spaces::boards::models::space_post_comment_like::SpacePostCommentLike;
use crate::spaces::SpacePostPath;
use crate::spaces::SpacePostPathParam;
use crate::*;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceCommentsQueryParams {
    pub bookmark: Option<String>,
}

pub async fn list_space_comments_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
    Query(ListSpaceCommentsQueryParams { bookmark }): Query<ListSpaceCommentsQueryParams>,
) -> Result<Json<ListItemsResponse<SpacePostCommentResponse>>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let mut opt = SpacePostCommentQueryOption::builder()
        .limit(10)
        .scan_index_forward(false);

    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (comments, bookmark) =
        SpacePostComment::find_by_post_order_by_likes(&dynamo.client, space_post_pk.clone(), opt)
            .await?;

    let comments: Vec<_> = comments
        .into_iter()
        .filter(|c| matches!(c.sk, EntityType::SpacePostComment(_)))
        .collect();

    let like_keys: Vec<_> = comments.iter().map(|c| c.like_keys(&user.pk)).collect();

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
