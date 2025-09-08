use crate::{Error, Result};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoProfile {
    // DynamoDB Keys
    pub pk: String, // USER#{user_id}
    pub sk: String, // PROFILE

    // Update-syntax fields (frequently updated)
    pub user_id: i64,
    pub profile_url: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub updated_at: i64,
}

impl DynamoProfile {
    pub fn new(user_id: i64) -> Self {
        let now = chrono::Utc::now().timestamp();

        Self {
            pk: format!("USER#{}", user_id),
            sk: "PROFILE".to_string(),
            user_id,
            profile_url: None,
            display_name: None,
            bio: None,
            updated_at: now,
        }
    }

    pub fn to_dynamo_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("PK".to_string(), AttributeValue::S(self.pk.clone()));
        item.insert("SK".to_string(), AttributeValue::S(self.sk.clone()));
        item.insert(
            "user_id".to_string(),
            AttributeValue::N(self.user_id.to_string()),
        );
        
        if let Some(profile_url) = &self.profile_url {
            item.insert("profile_url".to_string(), AttributeValue::S(profile_url.clone()));
        }
        
        if let Some(display_name) = &self.display_name {
            item.insert("display_name".to_string(), AttributeValue::S(display_name.clone()));
        }
        
        if let Some(bio) = &self.bio {
            item.insert("bio".to_string(), AttributeValue::S(bio.clone()));
        }
        
        item.insert(
            "updated_at".to_string(),
            AttributeValue::N(self.updated_at.to_string()),
        );

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
            sk: get_string("SK")?,
            user_id: get_number("user_id")?,
            profile_url: get_optional_string("profile_url"),
            display_name: get_optional_string("display_name"),
            bio: get_optional_string("bio"),
            updated_at: get_number("updated_at")?,
        })
    }
}
