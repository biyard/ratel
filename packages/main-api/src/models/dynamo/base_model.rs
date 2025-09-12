use aws_sdk_dynamodb::types::AttributeValue;
use dto::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseModel {
    pub pk: String,
    pub sk: String,
    pub entity_type: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub gsi1_pk: Option<String>,
    pub gsi1_sk: Option<String>,
    pub gsi2_pk: Option<String>,
    pub gsi2_sk: Option<String>,
}

impl BaseModel {
    pub fn new(pk: String, sk: String, entity_type: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            pk,
            sk,
            entity_type,
            created_at: now,
            updated_at: now,
            gsi1_pk: None,
            gsi1_sk: None,
            gsi2_pk: None,
            gsi2_sk: None,
        }
    }

    pub fn with_gsi1(mut self, pk: String, sk: Option<String>) -> Self {
        self.gsi1_pk = Some(pk);
        self.gsi1_sk = sk;
        self
    }

    pub fn with_gsi2(mut self, pk: String, sk: Option<String>) -> Self {
        self.gsi2_pk = Some(pk);
        self.gsi2_sk = sk;
        self
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

pub trait DynamoModel {
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>>;
    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self>
    where
        Self: Sized;
    fn pk(&self) -> String;
    fn sk(&self) -> String;
}

// Helper functions for AttributeValue conversion
pub fn string_attr(value: &str) -> AttributeValue {
    AttributeValue::S(value.to_string())
}

pub fn number_attr(value: i64) -> AttributeValue {
    AttributeValue::N(value.to_string())
}

pub fn bool_attr(value: bool) -> AttributeValue {
    AttributeValue::Bool(value)
}

pub fn optional_string_attr(value: &Option<String>) -> Option<AttributeValue> {
    value.as_ref().map(|v| AttributeValue::S(v.clone()))
}

pub fn optional_number_attr(value: &Option<i64>) -> Option<AttributeValue> {
    value.map(|v| AttributeValue::N(v.to_string()))
}

pub fn list_attr(values: &[String]) -> AttributeValue {
    let items: Vec<AttributeValue> = values.iter().map(|v| string_attr(v)).collect();
    AttributeValue::L(items)
}

// Extraction helpers
pub fn extract_string(item: &HashMap<String, AttributeValue>, key: &str) -> Result<String> {
    item.get(key)
        .and_then(|v| v.as_s().ok())
        .map(|s| s.clone())
        .ok_or_else(|| Error::DynamoDbError(format!("Missing or invalid string field: {}", key)))
}

pub fn extract_number(item: &HashMap<String, AttributeValue>, key: &str) -> Result<i64> {
    item.get(key)
        .and_then(|v| v.as_n().ok())
        .and_then(|s| s.parse::<i64>().ok())
        .ok_or_else(|| Error::DynamoDbError(format!("Missing or invalid number field: {}", key)))
}

pub fn extract_bool(item: &HashMap<String, AttributeValue>, key: &str) -> Result<bool> {
    item.get(key)
        .and_then(|v| v.as_bool().ok())
        .copied()
        .ok_or_else(|| Error::DynamoDbError(format!("Missing or invalid bool field: {}", key)))
}

pub fn extract_optional_string(
    item: &HashMap<String, AttributeValue>,
    key: &str,
) -> Option<String> {
    item.get(key).and_then(|v| v.as_s().ok()).cloned()
}

pub fn extract_optional_number(item: &HashMap<String, AttributeValue>, key: &str) -> Option<i64> {
    item.get(key)
        .and_then(|v| v.as_n().ok())
        .and_then(|s| s.parse::<i64>().ok())
}

pub fn extract_list_strings(
    item: &HashMap<String, AttributeValue>,
    key: &str,
) -> Vec<String> {
    item.get(key)
        .and_then(|v| v.as_l().ok())
        .map(|list| {
            list.iter()
                .filter_map(|item| item.as_s().ok().cloned())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default()
}

// Constants for key patterns
pub const USER_PREFIX: &str = "USER";
pub const SPACE_PREFIX: &str = "SPACE";
pub const FEED_PREFIX: &str = "FEED";
pub const GROUP_PREFIX: &str = "GROUP";
pub const DISCUSSION_PREFIX: &str = "DISCUSSION";
pub const METADATA_SK: &str = "METADATA";
pub const MEMBER_PREFIX: &str = "MEMBER";
pub const FOLLOWER_PREFIX: &str = "FOLLOWER";
pub const FOLLOWING_PREFIX: &str = "FOLLOWING";
pub const LIKE_PREFIX: &str = "LIKE";
pub const BOOKMARK_PREFIX: &str = "BOOKMARK";
pub const COMMENT_PREFIX: &str = "COMMENT";
pub const BADGE_PREFIX: &str = "BADGE";
pub const INDUSTRY_PREFIX: &str = "INDUSTRY";