use super::base_model::*;
use aws_sdk_dynamodb::types::AttributeValue;
use dto::{Result, Group};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoGroup {
    pub base: BaseModel,
    pub id: i64,
    pub name: String,
    pub description: String,
    pub permissions: i64,
    pub member_count: i64,
}

impl DynamoGroup {
    pub fn from_postgres_group(group: &Group) -> Self {
        let pk = format!("{}#{}", GROUP_PREFIX, group.id);
        let sk = METADATA_SK.to_string();
        
        let base = BaseModel::new(pk, sk, "GROUP".to_string())
            .with_gsi1(format!("GROUP_BY_NAME#{}", group.name), None);

        Self {
            base,
            id: group.id,
            name: group.name.clone(),
            description: group.description.clone(),
            permissions: group.permissions,
            member_count: 0, // Will be calculated during migration
        }
    }
}

impl DynamoModel for DynamoGroup {
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
        
        item.insert("id".to_string(), number_attr(self.id));
        item.insert("name".to_string(), string_attr(&self.name));
        item.insert("description".to_string(), string_attr(&self.description));
        item.insert("permissions".to_string(), number_attr(self.permissions));
        item.insert("member_count".to_string(), number_attr(self.member_count));

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
            permissions: extract_number(&item, "permissions")?,
            member_count: extract_number(&item, "member_count")?,
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
pub struct GroupMember {
    pub base: BaseModel,
    pub group_id: i64,
    pub user_id: i64,
    pub user_nickname: String,
    pub user_profile_url: Option<String>,
    pub joined_at: i64,
}

impl GroupMember {
    pub fn new(group_id: i64, user_id: i64, user_nickname: String, user_profile_url: Option<String>) -> Self {
        let pk = format!("{}#{}", GROUP_PREFIX, group_id);
        let sk = format!("{}#{}", MEMBER_PREFIX, user_id);
        let joined_at = chrono::Utc::now().timestamp();
        
        let base = BaseModel::new(pk, sk, "GROUP_MEMBER".to_string())
            .with_gsi1(format!("{}#{}", USER_PREFIX, user_id), Some(format!("{}#{}", GROUP_PREFIX, group_id)));

        Self {
            base,
            group_id,
            user_id,
            user_nickname,
            user_profile_url,
            joined_at,
        }
    }
}

impl DynamoModel for GroupMember {
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
        
        item.insert("group_id".to_string(), number_attr(self.group_id));
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
            group_id: extract_number(&item, "group_id")?,
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