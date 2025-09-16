use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct Post {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "INFO", name = "find_by_info", index = "gsi1", pk)]
    pub title: String,
    pub html_contents: String,
    pub feed_type: FeedType,
    pub status: FeedStatus,

    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    // only for reward spaces
    pub rewards: Option<i64>,
}

impl Post {
    pub fn new<T: Into<String>>(title: T, html_contents: T) -> Self {
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
            rewards: None,
        }
    }
}
