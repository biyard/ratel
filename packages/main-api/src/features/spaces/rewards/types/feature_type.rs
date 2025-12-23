use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::*;

#[derive(
    Debug,
    Clone,
    DynamoEnum,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
    OperationIo,
    Eq,
    PartialEq,
    Default,
)]
pub enum FeatureType {
    #[default]
    None,
    Poll,
}

impl From<EntityType> for FeatureType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::SpacePoll(_) => FeatureType::Poll,
            _ => FeatureType::None,
        }
    }
}

impl FeatureType {
    pub fn get_sk_prefix(&self) -> String {
        match self {
            FeatureType::None => "".to_string(),
            FeatureType::Poll => "POLL".to_string(),
        }
    }
}
