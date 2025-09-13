use super::base_model::*;
use super::serde_helpers as sh;
use crate::types::dynamo_entity_type::EntityType;
use dto::{Space, SpaceType, SpaceStatus, BoosterType, PublishingScope, File, NoticeQuestion, SpaceMember as PostgresSpaceMember};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoSpace {
    #[serde(flatten)]
    pub base: BaseModel,
    pub id: i64,
    pub title: Option<String>,
    pub html_contents: String,
    #[serde(with = "sh::space_type_num")]
    pub space_type: SpaceType,
    pub owner_id: i64,
    pub industry_id: i64,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub feed_id: i64,
    #[serde(with = "sh::space_status_num")]
    pub status: SpaceStatus,
    pub files: Vec<File>,
    pub num_of_redeem_codes: i64,
    pub likes: i64,
    pub shares: i64,
    pub is_liked: bool,
    pub rewards: i64,
    pub is_bookmarked: bool,
    pub number_of_comments: i64,
    pub image_url: Option<String>,
    pub notice_quiz: Vec<NoticeQuestion>,
    #[serde(with = "sh::booster_type_opt_num")]
    pub booster_type: Option<BoosterType>,
    #[serde(with = "sh::publishing_scope_num")]
    pub publishing_scope: PublishingScope,
    // Denormalized fields for quick access
    pub owner_nickname: String,
    pub owner_profile_url: Option<String>,
    pub industry_name: String,
}

impl DynamoSpace {
    pub fn from_postgres_space(space: &Space, owner_nickname: String, owner_profile_url: Option<String>, industry_name: String) -> Self {
        let pk = format!("{}#{}", SPACE_PREFIX, space.id);
        let sk = METADATA_SK.to_string();
        
        let base = BaseModel::new(pk, sk, EntityType::Space)
            .with_gsi1(format!("{}#{}", USER_PREFIX, space.owner_id), Some(format!("SPACE#{}", space.created_at)))
            .with_gsi2(format!("{}#{}", INDUSTRY_PREFIX, space.industry_id), Some(format!("SPACE#{}", space.created_at)));

        Self {
            base,
            id: space.id,
            title: space.title.clone(),
            html_contents: space.html_contents.clone(),
            space_type: space.space_type,
            owner_id: space.owner_id,
            industry_id: space.industry_id,
            started_at: space.started_at,
            ended_at: space.ended_at,
            feed_id: space.feed_id,
            status: space.status,
            files: space.files.clone(),
            num_of_redeem_codes: space.num_of_redeem_codes,
            likes: space.likes,
            shares: space.shares,
            is_liked: space.is_liked,
            rewards: space.rewards,
            is_bookmarked: space.is_bookmarked,
            number_of_comments: space.number_of_comments,
            image_url: space.image_url.clone(),
            notice_quiz: space.notice_quiz.clone(),
            booster_type: space.booster_type,
            publishing_scope: space.publishing_scope,
            owner_nickname,
            owner_profile_url,
            industry_name,
        }
    }

    pub fn from_postgres_space_member(member: &PostgresSpaceMember) -> SpaceMember {
        SpaceMember::new(
            member.space_id,
            member.user_id,
            "Unknown".to_string(), // TODO: Need to fetch user nickname from user table
            None // TODO: Need to fetch user profile URL from user table
        )
    }
}

impl DynamoModel for DynamoSpace {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMember {
    #[serde(flatten)]
    pub base: BaseModel,
    pub space_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub user_profile_url: Option<String>,
    pub joined_at: i64,
}

impl SpaceMember {
    pub fn new(space_id: i64, user_id: i64, user_nickname: String, user_profile_url: Option<String>) -> Self {
        let pk = format!("{}#{}", SPACE_PREFIX, space_id);
        let sk = format!("{}#{}", MEMBER_PREFIX, user_id);
        let joined_at = chrono::Utc::now().timestamp();
        
        let base = BaseModel::new(pk, sk, EntityType::Member)
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("{}#{}", SPACE_PREFIX, space_id)));

        Self {
            base,
            space_id,
            user_id,
            user_nickname,
            user_profile_url,
            joined_at,
        }
    }
}

impl DynamoModel for SpaceMember {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceLike {
    #[serde(flatten)]
    pub base: BaseModel,
    pub space_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub liked_at: i64,
}

impl SpaceLike {
    pub fn new(space_id: i64, user_id: i64, user_nickname: String) -> Self {
        let pk = format!("{}#{}", SPACE_PREFIX, space_id);
        let sk = format!("{}#{}", LIKE_PREFIX, user_id);
        let liked_at = chrono::Utc::now().timestamp();
        
        let base = BaseModel::new(pk, sk, EntityType::Like)
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("{}#{}", SPACE_PREFIX, space_id)));

        Self {
            base,
            space_id,
            user_id,
            user_nickname,
            liked_at,
        }
    }
}

impl DynamoModel for SpaceLike {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
