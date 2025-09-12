use super::base_model::*;
use aws_sdk_dynamodb::types::AttributeValue;
use dto::{Error, Result, Space, SpaceType, SpaceStatus, BoosterType, PublishingScope, File, NoticeQuestion, SpaceMember as PostgresSpaceMember};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoSpace {
    pub base: BaseModel,
    pub id: i64,
    pub title: Option<String>,
    pub html_contents: String,
    pub space_type: SpaceType,
    pub owner_id: i64,
    pub industry_id: i64,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub feed_id: i64,
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
    pub booster_type: Option<BoosterType>,
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
        
        let base = BaseModel::new(pk, sk, "SPACE".to_string())
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        
        // Base model fields
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));
        
        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        if let Some(ref gsi2_pk) = self.base.gsi2_pk {
            item.insert("gsi2_pk".to_string(), string_attr(gsi2_pk));
        }
        if let Some(ref gsi2_sk) = self.base.gsi2_sk {
            item.insert("gsi2_sk".to_string(), string_attr(gsi2_sk));
        }

        // Space-specific fields
        item.insert("id".to_string(), number_attr(self.id));
        
        if let Some(ref title) = self.title {
            item.insert("title".to_string(), string_attr(title));
        }
        
        item.insert("html_contents".to_string(), string_attr(&self.html_contents));
        item.insert("space_type".to_string(), number_attr(self.space_type as i64));
        item.insert("owner_id".to_string(), number_attr(self.owner_id));
        item.insert("industry_id".to_string(), number_attr(self.industry_id));
        
        if let Some(started_at) = self.started_at {
            item.insert("started_at".to_string(), number_attr(started_at));
        }
        
        if let Some(ended_at) = self.ended_at {
            item.insert("ended_at".to_string(), number_attr(ended_at));
        }
        
        item.insert("feed_id".to_string(), number_attr(self.feed_id));
        item.insert("status".to_string(), number_attr(self.status as i64));
        
        // Serialize files as JSON
        let files_json = serde_json::to_string(&self.files)
            .map_err(|e| Error::DynamoDbError(format!("Failed to serialize files: {}", e)))?;
        item.insert("files".to_string(), string_attr(&files_json));
        
        item.insert("num_of_redeem_codes".to_string(), number_attr(self.num_of_redeem_codes));
        item.insert("likes".to_string(), number_attr(self.likes));
        item.insert("shares".to_string(), number_attr(self.shares));
        item.insert("is_liked".to_string(), bool_attr(self.is_liked));
        item.insert("rewards".to_string(), number_attr(self.rewards));
        item.insert("is_bookmarked".to_string(), bool_attr(self.is_bookmarked));
        item.insert("number_of_comments".to_string(), number_attr(self.number_of_comments));
        
        if let Some(ref image_url) = self.image_url {
            item.insert("image_url".to_string(), string_attr(image_url));
        }
        
        // Serialize notice_quiz as JSON
        let notice_quiz_json = serde_json::to_string(&self.notice_quiz)
            .map_err(|e| Error::DynamoDbError(format!("Failed to serialize notice_quiz: {}", e)))?;
        item.insert("notice_quiz".to_string(), string_attr(&notice_quiz_json));
        
        if let Some(booster_type) = self.booster_type {
            item.insert("booster_type".to_string(), number_attr(booster_type as i64));
        }
        
        item.insert("publishing_scope".to_string(), number_attr(self.publishing_scope as i64));
        
        // Denormalized fields
        item.insert("owner_nickname".to_string(), string_attr(&self.owner_nickname));
        
        if let Some(ref owner_profile_url) = self.owner_profile_url {
            item.insert("owner_profile_url".to_string(), string_attr(owner_profile_url));
        }
        
        item.insert("industry_name".to_string(), string_attr(&self.industry_name));

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;
        
        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        let space_type_num = extract_number(&item, "space_type")?;
        let space_type = match space_type_num {
            1 => SpaceType::Legislation,
            2 => SpaceType::Poll,
            3 => SpaceType::Deliberation,
            4 => SpaceType::Nft,
            5 => SpaceType::Commitee,
            6 => SpaceType::SprintLeague,
            7 => SpaceType::Notice,
            8 => SpaceType::Dagit,
            _ => SpaceType::Legislation,
        };

        let status_num = extract_number(&item, "status")?;
        let status = match status_num {
            1 => SpaceStatus::Draft,
            2 => SpaceStatus::InProgress,
            3 => SpaceStatus::Finish,
            _ => SpaceStatus::Draft,
        };

        let publishing_scope_num = extract_number(&item, "publishing_scope")?;
        let publishing_scope = match publishing_scope_num {
            1 => PublishingScope::Private,
            2 => PublishingScope::Public,
            _ => PublishingScope::Private,
        };

        let booster_type = extract_optional_number(&item, "booster_type").map(|b| match b {
            1 => BoosterType::NoBoost,
            2 => BoosterType::X2,
            3 => BoosterType::X10,
            4 => BoosterType::X100,
            _ => BoosterType::NoBoost,
        });

        // Deserialize files from JSON
        let files_json = extract_string(&item, "files")?;
        let files: Vec<File> = serde_json::from_str(&files_json)
            .map_err(|e| Error::DynamoDbError(format!("Failed to deserialize files: {}", e)))?;

        // Deserialize notice_quiz from JSON
        let notice_quiz_json = extract_string(&item, "notice_quiz")?;
        let notice_quiz: Vec<NoticeQuestion> = serde_json::from_str(&notice_quiz_json)
            .map_err(|e| Error::DynamoDbError(format!("Failed to deserialize notice_quiz: {}", e)))?;

        Ok(Self {
            base,
            id: extract_number(&item, "id")?,
            title: extract_optional_string(&item, "title"),
            html_contents: extract_string(&item, "html_contents")?,
            space_type,
            owner_id: extract_number(&item, "owner_id")?,
            industry_id: extract_number(&item, "industry_id")?,
            started_at: extract_optional_number(&item, "started_at"),
            ended_at: extract_optional_number(&item, "ended_at"),
            feed_id: extract_number(&item, "feed_id")?,
            status,
            files,
            num_of_redeem_codes: extract_number(&item, "num_of_redeem_codes")?,
            likes: extract_number(&item, "likes")?,
            shares: extract_number(&item, "shares")?,
            is_liked: extract_bool(&item, "is_liked")?,
            rewards: extract_number(&item, "rewards")?,
            is_bookmarked: extract_bool(&item, "is_bookmarked")?,
            number_of_comments: extract_number(&item, "number_of_comments")?,
            image_url: extract_optional_string(&item, "image_url"),
            notice_quiz,
            booster_type,
            publishing_scope,
            owner_nickname: extract_string(&item, "owner_nickname")?,
            owner_profile_url: extract_optional_string(&item, "owner_profile_url"),
            industry_name: extract_string(&item, "industry_name")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMember {
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
        
        let base = BaseModel::new(pk, sk, "SPACE_MEMBER".to_string())
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));
        
        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        
        item.insert("space_id".to_string(), number_attr(self.space_id));
        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("user_nickname".to_string(), string_attr(&self.user_nickname));
        
        if let Some(ref url) = self.user_profile_url {
            item.insert("user_profile_url".to_string(), string_attr(url));
        }
        
        item.insert("joined_at".to_string(), number_attr(self.joined_at));

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;
        
        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        Ok(Self {
            base,
            space_id: extract_number(&item, "space_id")?,
            user_id: extract_number(&item, "user_id")?,
            user_nickname: extract_string(&item, "user_nickname")?,
            user_profile_url: extract_optional_string(&item, "user_profile_url"),
            joined_at: extract_number(&item, "joined_at")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceLike {
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
        
        let base = BaseModel::new(pk, sk, "SPACE_LIKE".to_string())
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));
        
        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        
        item.insert("space_id".to_string(), number_attr(self.space_id));
        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("user_nickname".to_string(), string_attr(&self.user_nickname));
        item.insert("liked_at".to_string(), number_attr(self.liked_at));

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;
        
        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        Ok(Self {
            base,
            space_id: extract_number(&item, "space_id")?,
            user_id: extract_number(&item, "user_id")?,
            user_nickname: extract_string(&item, "user_nickname")?,
            liked_at: extract_number(&item, "liked_at")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}