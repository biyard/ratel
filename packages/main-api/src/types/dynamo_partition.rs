use bdk::prelude::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum)]
#[dynamo_enum(error = "crate::Error2")]
pub enum Partition {
    #[default]
    None,

    User(String),
    Email(String),
    Feed(String),
    Space(String),
    Team(String),
}

impl Partition {
    pub fn key<T: Display>(self, id: T) -> String {
        format!("{}#{}", self, id)
    }
}

// Manually checking the generated impl to compare with macro
// This should generate the exact same code as our manual implementation
// impl std::str::FromStr for Partition {
//     type Err = crate::Error2;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(match s {
//             s if s.starts_with("USER#") => Partition::User(s.to_string()),
//             s if s.starts_with("EMAIL#") => Partition::Email(s.to_string()),
//             s if s.starts_with("FEED#") => Partition::Feed(s.to_string()),
//             s if s.starts_with("SPACE#") => Partition::Space(s.to_string()),
//             s if s.starts_with("TEAM#") => Partition::Team(s.to_string()),
//             _ => Err(crate::Error2::InvalidPartitionKey(s.to_string()))?,
//         })
//     }
// }
