use crate::AppState;
use crate::controllers::v3::spaces::{SpacePostCommentPath, SpacePostCommentPathParam};
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::feed::PostComment;
use crate::models::user::User;
use crate::types::{ListItemsQuery, ListItemsResponse, Partition, TeamGroupPermission};
use crate::Permissions;
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

pub async fn list_space_comments_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(_user): NoApi<User>,
    Path(SpacePostCommentPathParam {
        space_pk,
        space_post_pk,
        space_post_comment_sk,
    }): SpacePostCommentPath,
    Query(_query): ListItemsQuery,
) -> Result<Json<ListItemsResponse<SpacePostComment>>, crate::Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(crate::Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(crate::Error::NoPermission);
    }
    let comments =
        SpacePostComment::list_by_comment(&dynamo.client, space_post_pk, space_post_comment_sk)
            .await?;

    // TODO: compose with comment like

    Ok(Json(comments.into()))
}
