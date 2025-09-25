use crate::{
    models::{team::Team, user::User},
    types::*,
};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity, JsonSchema)]
pub struct Post {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi6", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    pub html_contents: String,
    pub post_type: PostType,

    pub status: PostStatus,

    #[dynamo(index = "gsi6", name = "find_by_visibility", pk)]
    pub visibility: Option<Visibility>,

    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    #[dynamo(
        prefix = "USER_VISIBILITY",
        name = "find_by_user_pk_visibility",
        index = "gsi2",
        pk
    )]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    // #[dynamo(prefix = "SPACE_PK", name = "find_by_space_pk", index = "gsi2", pk)]
    pub space_pk: Option<Partition>,
    pub booster: Option<BoosterType>,
    // only for reward spaces
    pub rewards: Option<i64>,

    // Only for list posts Composed key
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub compose_sort_key: String,
}

impl Post {
    pub fn get_compose_key(status: PostStatus, visibility: Option<Visibility>, now: i64) -> String {
        match (status, visibility) {
            (PostStatus::Draft, _) => format!("DRAFT#{}", now),
            (PostStatus::Published, Some(Visibility::Public)) => format!("PUBLIC#{}", now),
            (PostStatus::Published, Some(Visibility::Team(team_pk))) => {
                format!("TEAM#{}#{}", team_pk, now)
            }
            _ => format!("DRAFT#{}", now), // Fallback to Draft key
        }
    }
    pub fn new<T: Into<String>, A: Into<Author>>(
        title: T,
        html_contents: T,
        post_type: PostType,
        author: A,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp_micros();
        let Author {
            pk,
            display_name,
            profile_url,
            username,
        } = author.into();

        Self {
            pk: Partition::Feed(uid),
            sk: EntityType::Post,
            created_at: now,
            updated_at: now,
            post_type,
            title: title.into(),
            html_contents: html_contents.into(),
            status: PostStatus::Draft,
            visibility: None,
            shares: 0,
            likes: 0,
            comments: 0,

            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,

            space_pk: None,
            booster: None,
            rewards: None,
            compose_sort_key: Self::get_compose_key(PostStatus::Draft, None, now),
        }
    }
}

pub struct Author {
    pub pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl From<User> for Author {
    fn from(
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        Self {
            pk,
            display_name,
            profile_url,
            username,
        }
    }
}
impl From<Team> for Author {
    fn from(
        Team {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: Team,
    ) -> Self {
        Self {
            pk,
            display_name,
            profile_url,
            username,
        }
    }
}

pub struct PostResponse {
    pub pk: Partition,
    pub title: String,
    pub html_contents: String,
    pub post_type: PostType,
    pub status: PostStatus,
    pub visibility: Option<Visibility>,
    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub space_pk: Option<Partition>,
    pub booster: Option<BoosterType>,
    pub rewards: Option<i64>,
}

impl From<Post> for PostResponse {
    fn from(
        Post {
            pk,
            title,
            html_contents,
            post_type,
            status,
            visibility,
            shares,
            likes,
            comments,
            user_pk,
            author_display_name,
            author_profile_url,
            author_username,
            space_pk,
            booster,
            rewards,
            ..
        }: Post,
    ) -> Self {
        Self {
            pk,
            title,
            html_contents,
            post_type,
            status,
            visibility,
            shares,
            likes,
            comments,
            user_pk,
            author_display_name,
            author_profile_url,
            author_username,
            space_pk,
            booster,
            rewards,
        }
    }
}
