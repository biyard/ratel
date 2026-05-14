use crate::features::posts::models::Post;
use crate::features::posts::types::*;
use crate::features::posts::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct PostResponse {
    pub pk: FeedPartition,

    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    #[serde(alias = "html_contents")]
    pub body: ContentBody,

    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
    pub auth_pk: Partition,
    pub author_type: crate::features::auth::UserType,

    pub space_pk: Option<Partition>,
    pub space_type: Option<SpaceType>,
    pub booster: BoosterType,
    pub rewards: Option<i64>,

    pub urls: Vec<String>,
    pub liked: bool,
    #[serde(default)]
    pub categories: Vec<String>,
    pub status: PostStatus,

    /// Echoes back the underlying Post's `announcement_id` for client
    /// UIs that want to badge broadcast cards differently. The single
    /// anchor Post for a sub-team announcement lives at
    /// `Feed(announcement_id)`, so `self.pk` already IS the canonical
    /// URL — no separate routing branch needed.
    #[serde(default)]
    pub announcement_id: Option<String>,
}

impl PostResponse {
    pub fn url(&self) -> Route {
        if let Some(space_pk) = &self.space_pk {
            Route::SpaceIndexPage {
                space_id: space_pk.clone().into(),
            }
        } else {
            Route::PostDetail {
                post_id: self.pk.clone().into(),
            }
        }
    }

    pub fn has_space(&self) -> bool {
        self.space_pk.is_some()
    }

    pub fn with_like(mut self, liked: bool) -> Self {
        self.liked = liked;
        self
    }
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        PostResponse {
            pk: post.pk.into(),
            created_at: post.created_at,
            updated_at: post.updated_at,
            title: post.title,
            body: post.body,
            shares: post.shares,
            likes: post.likes,
            comments: post.comments,
            author_display_name: post.author_display_name,
            author_profile_url: post.author_profile_url,
            author_username: post.author_username,
            space_pk: post.space_pk,
            booster: post.booster.unwrap_or(BoosterType::NoBoost),
            rewards: post.rewards,
            urls: post.urls.clone(),
            liked: false,
            auth_pk: post.user_pk,
            space_type: post.space_type,
            author_type: post.author_type,
            categories: post.categories,
            status: post.status,
            announcement_id: post.announcement_id,
        }
    }
}
