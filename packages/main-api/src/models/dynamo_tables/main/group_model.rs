use super::base_model::*;
use crate::types::dynamo_entity_type::EntityType;
use dto::Group;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoGroup {
    #[serde(flatten)]
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
        
        let base = BaseModel::new(pk, sk, EntityType::Group)
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
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    #[serde(flatten)]
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
        
        let base = BaseModel::new(pk, sk, EntityType::Member)
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
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
