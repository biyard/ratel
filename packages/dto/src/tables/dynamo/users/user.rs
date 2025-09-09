use crate::{Error, Result};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::UserSortKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoUser {
    // DynamoDB Keys
    pub pk: String, // USER#{user_id}
    pub sk: UserSortKey, // USER

    // Basic User fields (not frequently updated)
    pub user_id: i64,
    pub telegram_id: Option<String>,
    pub evm_address: Option<String>,
    pub username: String,
    pub created_at: i64,

    // GSI fields
    pub gsi1_pk: Option<String>, // For username lookup
    pub gsi1_sk: Option<String>,
}

impl DynamoUser {
    pub fn new(
        user_id: i64,
        username: String,
        telegram_id: Option<String>,
        evm_address: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();

        Self {
            pk: format!("USER#{}", user_id),
            sk: UserSortKey::User,
            user_id,
            telegram_id,
            evm_address,
            username: username.clone(),
            created_at: now,
            gsi1_pk: Some(format!("USERNAME#{}", username)),
            gsi1_sk: Some("USER".to_string()),
        }
    }

    pub fn to_dynamo_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("PK".to_string(), AttributeValue::S(self.pk.clone()));
        item.insert("SK".to_string(), AttributeValue::S(self.sk.to_string()));

        // Basic User fields
        item.insert(
            "user_id".to_string(),
            AttributeValue::N(self.user_id.to_string()),
        );

        if let Some(telegram_id) = &self.telegram_id {
            item.insert(
                "telegram_id".to_string(),
                AttributeValue::S(telegram_id.clone()),
            );
        }

        if let Some(evm_address) = &self.evm_address {
            item.insert(
                "evm_address".to_string(),
                AttributeValue::S(evm_address.clone()),
            );
        }

        item.insert(
            "username".to_string(),
            AttributeValue::S(self.username.clone()),
        );
        item.insert(
            "created_at".to_string(),
            AttributeValue::N(self.created_at.to_string()),
        );

        // GSI fields
        if let Some(gsi1_pk) = &self.gsi1_pk {
            item.insert("GSI1_PK".to_string(), AttributeValue::S(gsi1_pk.clone()));
        }

        if let Some(gsi1_sk) = &self.gsi1_sk {
            item.insert("GSI1_SK".to_string(), AttributeValue::S(gsi1_sk.clone()));
        }

        item
    }

    pub fn from_dynamo_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let get_string = |key: &str| -> Result<String> {
            item.get(key)
                .and_then(|v| match v {
                    AttributeValue::S(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::DynamoDbSerializationError(format!("Missing or invalid {}", key))
                })
        };

        let get_optional_string = |key: &str| -> Option<String> {
            item.get(key).and_then(|v| match v {
                AttributeValue::S(s) => Some(s.clone()),
                _ => None,
            })
        };

        let get_number = |key: &str| -> Result<i64> {
            item.get(key)
                .and_then(|v| match v {
                    AttributeValue::N(n) => n.parse().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    Error::DynamoDbSerializationError(format!("Missing or invalid {}", key))
                })
        };

        Ok(Self {
            pk: get_string("PK")?,
            sk: UserSortKey::from(get_string("SK")?.as_str()),
            user_id: get_number("user_id")?,
            telegram_id: get_optional_string("telegram_id"),
            evm_address: get_optional_string("evm_address"),
            username: get_string("username")?,
            created_at: get_number("created_at")?,
            gsi1_pk: get_optional_string("GSI1_PK"),
            gsi1_sk: get_optional_string("GSI1_SK"),
        })
    }
}
