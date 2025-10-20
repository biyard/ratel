use crate::{
    models::{User, feed::Post},
    types::{BoosterType, Partition, SpaceType},
};
use bdk::prelude::*;

#[derive(
    Debug, Clone, Default, serde::Deserialize, serde::Serialize, JsonSchema, aide::OperationIo,
)]
pub struct PostResponse {
    pub pk: Partition,

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
    pub auth_pk: Partition,
    pub author_type: crate::types::UserType,

    pub space_pk: Option<Partition>,
    pub space_type: Option<SpaceType>,
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
        let (space_pk, space_type) = match post.space_visibility {
            Some(crate::types::SpaceVisibility::Public) => {
                (post.space_pk.clone(), post.space_type.clone())
            }
            _ => (None, None),
        };

        PostResponse {
            pk: post.pk,
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
            space_pk,
            booster: post.booster.unwrap_or(BoosterType::NoBoost),
            rewards: post.rewards,
            urls: post.urls.clone(),
            liked: false,
            auth_pk: post.user_pk,
            space_type,
            author_type: post.author_type,
        }
    }
}

impl From<(Option<User>, Post)> for PostResponse {
    fn from((user, post): (Option<User>, Post)) -> Self {
        let (space_pk, space_type) = match (user, post.space_visibility.clone()) {
            (_, Some(crate::types::SpaceVisibility::Public)) => {
                (post.space_pk.clone(), post.space_type.clone())
            }
            (Some(user), _) if user.pk == post.user_pk => {
                (post.space_pk.clone(), post.space_type.clone())
            }
            _ => (None, None),
        };

        PostResponse {
            pk: post.pk,
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
            booster: post.booster.unwrap_or(BoosterType::NoBoost),
            rewards: post.rewards,
            urls: post.urls.clone(),
            liked: false,
            auth_pk: post.user_pk,
            author_type: post.author_type,

            space_pk,
            space_type,
        }
    }
}
