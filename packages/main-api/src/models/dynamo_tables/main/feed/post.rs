use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
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
    pub feed_type: FeedType,
    pub status: FeedStatus,

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
    pub fn new<T: Into<String>>(
        title: T,
        html_contents: T,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk: Partition::Feed(uid),
            sk: EntityType::Post,
            created_at,
            updated_at: created_at,
            feed_type: FeedType::Post,
            title: title.into(),
            html_contents: html_contents.into(),
            status: FeedStatus::Draft,
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

impl Post {
    pub fn update_builder() -> PostUpdateBuilder {
        PostUpdateBuilder {}
    }
}

pub struct PostUpdateBuilder {}
