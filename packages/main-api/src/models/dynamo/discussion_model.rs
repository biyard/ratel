use super::base_model::*;
use aws_sdk_dynamodb::types::AttributeValue;
use dto::{Result, Discussion};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoDiscussion {
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
        author_profile_url: Option<String>
    ) -> Self {
        let pk = format!("{}#{}", DISCUSSION_PREFIX, discussion.id);
        let sk = METADATA_SK.to_string();
        
        let base = BaseModel::new(pk, sk, "DISCUSSION".to_string())
            .with_gsi1(format!("{}#{}", SPACE_PREFIX, discussion.space_id), Some(format!("DISCUSSION#{}", discussion.created_at)))
            .with_gsi2(format!("{}#{}", USER_PREFIX, discussion.creator_id), Some(format!("DISCUSSION#{}", discussion.created_at)));

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
        if let Some(ref gsi2_pk) = self.base.gsi2_pk {
            item.insert("gsi2_pk".to_string(), string_attr(gsi2_pk));
        }
        if let Some(ref gsi2_sk) = self.base.gsi2_sk {
            item.insert("gsi2_sk".to_string(), string_attr(gsi2_sk));
        }
        
        item.insert("id".to_string(), number_attr(self.id));
        item.insert("name".to_string(), string_attr(&self.name));
        item.insert("description".to_string(), string_attr(&self.description));
        item.insert("space_id".to_string(), number_attr(self.space_id));
        item.insert("creator_id".to_string(), number_attr(self.creator_id));
        item.insert("participant_count".to_string(), number_attr(self.participant_count));
        
        if let Some(ref space_title) = self.space_title {
            item.insert("space_title".to_string(), string_attr(space_title));
        }
        
        item.insert("author_nickname".to_string(), string_attr(&self.author_nickname));
        
        if let Some(ref url) = self.author_profile_url {
            item.insert("author_profile_url".to_string(), string_attr(url));
        }

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
            id: extract_number(&item, "id")?,
            name: extract_string(&item, "name")?,
            description: extract_string(&item, "description")?,
            space_id: extract_number(&item, "space_id")?,
            creator_id: extract_number(&item, "creator_id")?,
            participant_count: extract_number(&item, "participant_count")?,
            space_title: extract_optional_string(&item, "space_title"),
            author_nickname: extract_string(&item, "author_nickname")?,
            author_profile_url: extract_optional_string(&item, "author_profile_url"),
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
pub struct DiscussionParticipant {
    pub base: BaseModel,
    pub discussion_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub user_profile_url: Option<String>,
    pub joined_at: i64,
}

impl DiscussionParticipant {
    pub fn new(discussion_id: i64, user_id: i64, user_nickname: String, user_profile_url: Option<String>) -> Self {
        let pk = format!("{}#{}", DISCUSSION_PREFIX, discussion_id);
        let sk = format!("{}#{}", MEMBER_PREFIX, user_id);
        let joined_at = chrono::Utc::now().timestamp();
        
        let base = BaseModel::new(pk, sk, "DISCUSSION_PARTICIPANT".to_string())
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("DISCUSSION#{}", discussion_id)));

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
        
        item.insert("discussion_id".to_string(), number_attr(self.discussion_id));
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
            discussion_id: extract_number(&item, "discussion_id")?,
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
pub struct DiscussionCommentItem {
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
        let sk = format!("{}#{}#{}", COMMENT_PREFIX, user_id, chrono::Utc::now().timestamp_millis());
        let commented_at = chrono::Utc::now().timestamp();
        
        let base = BaseModel::new(pk, sk, "DISCUSSION_COMMENT".to_string())
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("COMMENT#{}", commented_at)));

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
        
        item.insert("discussion_id".to_string(), number_attr(self.discussion_id));
        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("user_nickname".to_string(), string_attr(&self.user_nickname));
        item.insert("comment".to_string(), string_attr(&self.comment));
        item.insert("commented_at".to_string(), number_attr(self.commented_at));

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
            discussion_id: extract_number(&item, "discussion_id")?,
            user_id: extract_number(&item, "user_id")?,
            user_nickname: extract_string(&item, "user_nickname")?,
            comment: extract_string(&item, "comment")?,
            commented_at: extract_number(&item, "commented_at")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}