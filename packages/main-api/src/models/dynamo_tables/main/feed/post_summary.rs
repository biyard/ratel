use bdk::prelude::*;
use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostSummary {
    Post(Post),
    PostAuthor(PostAuthor),
    PostSpace(PostSpace),
}
