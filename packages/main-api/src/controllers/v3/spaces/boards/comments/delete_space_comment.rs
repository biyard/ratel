use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::{
    SpacePostComment, SpacePostCommentQueryOption,
};
use crate::features::spaces::boards::models::space_post_comment_like::SpacePostCommentLike;
use crate::models::SpaceCommon;
use crate::spaces::SpacePostCommentPath;
use crate::spaces::SpacePostCommentPathParam;
use crate::*;

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct DeleteSpaceCommentResponse {
    pub space_post_comment_sk: EntityType,
}

pub async fn delete_space_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePostCommentPathParam {
        space_pk,
        space_post_pk,
        space_post_comment_sk,
    }): SpacePostCommentPath,
    Extension(space): Extension<SpaceCommon>,
) -> Result<Json<DeleteSpaceCommentResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    if space.status == Some(SpaceStatus::Finished) {
        return Err(Error::FinishedSpace);
    }

    let comment = SpacePostComment::get(
        &dynamo.client,
        space_post_pk.clone(),
        Some(space_post_comment_sk.clone()),
    )
    .await?
    .ok_or(Error::PostCommentError)?;

    let is_owner = comment.author_pk == user.pk;
    if !is_owner {
        return Err(Error::NoPermission);
    }

    let mut tx_items = Vec::new();
    let mut deleted_count: i64 = 1;

    if matches!(comment.sk, EntityType::SpacePostComment(_)) {
        let mut bookmark: Option<String> = None;

        loop {
            let mut opt = SpacePostCommentQueryOption::builder()
                .limit(25)
                .scan_index_forward(false);

            if let Some(ref bm) = bookmark {
                opt = opt.bookmark(bm.clone());
            }

            let (replies, next_bookmark) =
                SpacePostComment::list_by_comment(&dynamo.client, comment.sk.clone(), opt).await?;

            for reply in replies {
                tx_items.push(SpacePostComment::delete_transact_write_item(
                    &space_post_pk,
                    &reply.sk,
                ));
                deleted_count += 1;
            }

            if next_bookmark.is_none() {
                break;
            }

            bookmark = next_bookmark;
        }
    }

    let (post_pk, post_sk) = SpacePost::keys(&space_pk, &space_post_pk);
    tx_items.push(
        SpacePost::updater(&post_pk, post_sk)
            .increase_comments(-deleted_count)
            .transact_write_item(),
    );

    if let Some(parent_sk) = comment.parent_comment_sk.clone() {
        tx_items.push(
            SpacePostComment::updater(&space_post_pk, &parent_sk)
                .increase_replies(-1)
                .transact_write_item(),
        );
    }

    tx_items.push(SpacePostComment::delete_transact_write_item(
        &space_post_pk,
        &space_post_comment_sk,
    ));

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(tx_items))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("failed to delete comment: {:?}", e);
            Error::PostCommentError
        })?;

    Ok(Json(DeleteSpaceCommentResponse {
        space_post_comment_sk,
    }))
}
