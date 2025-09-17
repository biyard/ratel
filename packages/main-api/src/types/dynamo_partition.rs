use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, strum_macros::Display, Default)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Partition {
    #[default]
    None,

    #[strum(to_string = "USER#{0}")]
    User(String),
    #[strum(to_string = "EMAIL#{0}")]
    Email(String),
    #[strum(to_string = "FEED#{0}")]
    Feed(String),
    #[strum(to_string = "SPACE#{0}")]
    Space(String),
    #[strum(to_string = "TEAM#{0}")]
    Team(String),
}

impl Partition {
    pub fn key<T: Display>(self, id: T) -> String {
        format!("{}#{}", self, id)
    }
}

impl FromStr for Partition {
    type Err = crate::Error2;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            s if s.starts_with("USER#") => Partition::User(s.to_string()),
            s if s.starts_with("EMAIL#") => Partition::Email(s.to_string()),
            s if s.starts_with("FEED#") => Partition::Feed(s.to_string()),
            s if s.starts_with("SPACE#") => Partition::Space(s.to_string()),
            s if s.starts_with("TEAM#") => Partition::Team(s.to_string()),
            _ => Err(crate::Error2::InvalidPartitionKey(s.to_string()))?,
        })
    }
}
