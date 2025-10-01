use crate::{
    models::feed::Post,
    types::{BoosterType, EntityType, Partition},
};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct PostResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    pub html_contents: String,

    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub space_pk: Option<Partition>,
    pub booster: BoosterType,
    // only for reward spaces
    pub rewards: Option<i64>,

    // Only for list posts Composed key
    pub urls: Vec<String>,
    pub liked: bool,
}

impl PostResponse {
    pub fn with_like(mut self, liked: bool) -> Self {
        self.liked = liked;
        self
    }
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        PostResponse {
            pk: post.pk,
            sk: post.sk,
            created_at: post.created_at,
            updated_at: post.updated_at,
            title: post.title,
            html_contents: post.html_contents,
            shares: post.shares,
            likes: post.likes,
            comments: post.comments,
            author_display_name: post.author_display_name,
            author_profile_url: post.author_profile_url,
            author_username: post.author_username,
            space_pk: post.space_pk.clone(),
            booster: post.booster.unwrap_or(BoosterType::NoBoost),
            rewards: post.rewards,
            urls: post.urls.clone(),
            liked: false,
        }
    }
}
