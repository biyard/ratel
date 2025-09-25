use crate::{
    models::{team::Team, user::User},
    types::*,
};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity, JsonSchema)]
pub struct Post {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi6", name = "find_posts", pk)]
    pub sk: EntityType,

    #[dynamo(index = "gsi6", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    pub html_contents: String,
    pub post_type: PostType,
    pub status: PostStatus,
    pub visibility: Visibility,

    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    #[dynamo(prefix = "SPACE_PK", name = "find_by_space_pk", index = "gsi2", pk)]
    pub space_pk: Option<Partition>,
    pub booster: Option<BoosterType>,
    // only for reward spaces
    pub rewards: Option<i64>,
}

impl Post {
    pub fn new<T: Into<String>, A: Into<PostAuthor>>(
        title: T,
        html_contents: T,
        post_type: PostType,
        author: A,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().timestamp_micros();
        let PostAuthor {
            pk,
            display_name,
            profile_url,
            username,
        } = author.into();

        Self {
            pk: Partition::Feed(uid),
            sk: EntityType::Post,
            created_at,
            updated_at: created_at,
            post_type,
            title: title.into(),
            html_contents: html_contents.into(),
            status: PostStatus::Draft,
            visibility: Visibility::Private,
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
        }
    }
}

pub struct PostAuthor {
    pub pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl From<User> for PostAuthor {
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
impl From<Team> for PostAuthor {
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
