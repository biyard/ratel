use crate::types::*;
use axum::extract::Path;
use bdk::prelude::*;

pub type PostCommentPath = Path<PostCommentPathParam>;

#[derive(Debug, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct PostCommentPathParam {
    pub post_pk: Partition,
    pub comment_sk: EntityType,
}
