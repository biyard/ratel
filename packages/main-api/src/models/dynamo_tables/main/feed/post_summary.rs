use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostSummary {
    Post(Post),
    PostAuthor(PostAuthor),
}
