use crate::types::*;
use axum::extract::Path;
use bdk::prelude::*;

pub type PostPath = Path<PostPathParam>;

#[derive(Debug, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct PostPathParam {
    pub post_pk: Partition,
}
