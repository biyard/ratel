use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::SpaceCommon;
use crate::spaces::SpacePostCommentPath;
use crate::spaces::SpacePostCommentPathParam;
use crate::*;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceCommentResponse {
    pub space_post_comment_sk: EntityType,
}

pub async fn update_space_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePostCommentPathParam {
        space_pk,
        space_post_pk,
        space_post_comment_sk,
    }): SpacePostCommentPath,
    Extension(space): Extension<SpaceCommon>,
    Json(UpdateSpaceCommentRequest { content }): Json<UpdateSpaceCommentRequest>,
) -> Result<Json<UpdateSpaceCommentResponse>> {
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

    let now = chrono::Utc::now().timestamp();
    let updated_at_align = format!("{:020}", now);

    let update_tx = SpacePostComment::updater(&comment.pk, &comment.sk)
        .with_content(content)
        .with_updated_at(now)
        .with_updated_at_align(updated_at_align)
        .transact_write_item();

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(vec![update_tx]))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("failed to update comment: {:?}", e);
            Error::PostCommentError
        })?;

    Ok(Json(UpdateSpaceCommentResponse {
        space_post_comment_sk,
    }))
}
