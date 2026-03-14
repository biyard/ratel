use crate::features::posts::models::Post;
use crate::features::posts::types::*;
use crate::features::posts::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PostResponse {
    pub pk: FeedPartition,

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
    pub author_type: crate::features::auth::UserType,

    pub space_pk: Option<Partition>,
    pub space_type: Option<SpaceType>,
    pub booster: BoosterType,
    pub rewards: Option<i64>,

    pub urls: Vec<String>,
    pub liked: bool,
}

impl PostResponse {
    pub fn url(&self) -> String {
        if let Some(space_pk) = &self.space_pk {
            format!(
                "/spaces/{}/dashboard",
                Into::<SpacePartition>::into(space_pk.clone())
            )
        } else {
            format!("/posts/{}", Into::<FeedPartition>::into(self.pk.clone()))
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
        let (space_pk, space_type) = match post.space_visibility {
            Some(SpaceVisibility::Public) => (post.space_pk.clone(), post.space_type.clone()),
            _ => (None, None),
        };

        PostResponse {
            pk: post.pk.into(),
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

impl From<(Option<crate::features::auth::User>, Post)> for PostResponse {
    fn from((user, post): (Option<crate::features::auth::User>, Post)) -> Self {
        let (space_pk, space_type) = match (user, post.space_visibility.clone()) {
            (_, Some(SpaceVisibility::Public)) => (post.space_pk.clone(), post.space_type.clone()),
            (Some(user), _) if user.pk == post.user_pk => {
                (post.space_pk.clone(), post.space_type.clone())
            }
            _ => (None, None),
        };

        PostResponse {
            pk: post.pk.into(),
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
