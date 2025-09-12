use super::base_model::*;
use crate::types::dynamo_entity_type::EntityType;
use dto::Discussion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoDiscussion {
    #[serde(flatten)]
    pub base: BaseModel,
    pub id: i64,
    pub name: String,
    pub description: String,
    pub space_id: i64,
    pub creator_id: i64,
    pub participant_count: i64,
    // Denormalized fields
    pub space_title: Option<String>,
    pub author_nickname: String,
    pub author_profile_url: Option<String>,
}

impl DynamoDiscussion {
    pub fn from_postgres_discussion(
        discussion: &Discussion,
        space_title: Option<String>,
        author_nickname: String,
        author_profile_url: Option<String>,
    ) -> Self {
        let pk = format!("{}#{}", DISCUSSION_PREFIX, discussion.id);
        let sk = METADATA_SK.to_string();

        let base = BaseModel::new(pk, sk, EntityType::Discussion)
            .with_gsi1(
                format!("{}#{}", SPACE_PREFIX, discussion.space_id),
                Some(format!("DISCUSSION#{}", discussion.created_at)),
            )
            .with_gsi2(
                format!("{}#{}", USER_PREFIX, discussion.creator_id),
                Some(format!("DISCUSSION#{}", discussion.created_at)),
            );

        Self {
            base,
            id: discussion.id,
            name: discussion.name.clone(),
            description: discussion.description.clone(),
            space_id: discussion.space_id,
            creator_id: discussion.creator_id,
            participant_count: 0, // Will be calculated during migration
            space_title,
            author_nickname,
            author_profile_url,
        }
    }

    // TODO: Add discussion comment conversion when the proper table structure is identified
}

impl DynamoModel for DynamoDiscussion {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionParticipant {
    #[serde(flatten)]
    pub base: BaseModel,
    pub discussion_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub user_profile_url: Option<String>,
    pub joined_at: i64,
}

impl DiscussionParticipant {
    pub fn new(
        discussion_id: i64,
        user_id: i64,
        user_nickname: String,
        user_profile_url: Option<String>,
    ) -> Self {
        let pk = format!("{}#{}", DISCUSSION_PREFIX, discussion_id);
        let sk = format!("{}#{}", MEMBER_PREFIX, user_id);
        let joined_at = chrono::Utc::now().timestamp();

        let base = BaseModel::new(pk, sk, EntityType::Member).with_gsi1(
            format!("{}#{}", USER_PREFIX, user_id),
            Some(format!("DISCUSSION#{}", discussion_id)),
        );

        Self {
            base,
            discussion_id,
            user_id,
            user_nickname,
            user_profile_url,
            joined_at,
        }
    }
}

impl DynamoModel for DiscussionParticipant {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionCommentItem {
    #[serde(flatten)]
    pub base: BaseModel,
    pub discussion_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub comment: String,
    pub commented_at: i64,
}

impl DiscussionCommentItem {
    pub fn new(discussion_id: i64, user_id: i64, user_nickname: String, comment: String) -> Self {
        let pk = format!("{}#{}", DISCUSSION_PREFIX, discussion_id);
        let sk = format!(
            "{}#{}#{}",
            COMMENT_PREFIX,
            user_id,
            chrono::Utc::now().timestamp_millis()
        );
        let commented_at = chrono::Utc::now().timestamp();

        let base = BaseModel::new(pk, sk, EntityType::Comment).with_gsi1(
            format!("{}#{}", USER_PREFIX, user_id),
            Some(format!("COMMENT#{}", commented_at)),
        );

        Self {
            base,
            discussion_id,
            user_id,
            user_nickname,
            comment,
            commented_at,
        }
    }
}

impl DynamoModel for DiscussionCommentItem {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
