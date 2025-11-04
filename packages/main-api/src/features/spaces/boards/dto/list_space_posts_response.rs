use crate::features::spaces::boards::dto::space_post_response::SpacePostResponse;
use bdk::prelude::*;
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ListSpacePostsResponse {
    pub posts: Vec<SpacePostResponse>,
    pub bookmark: Option<String>,
}
