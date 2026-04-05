use std::fmt::Display;
use std::str::FromStr;

use crate::common::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum AuthorPartition {
    #[default]
    Unknown,
    User(String),
    Team(String),
}

impl Display for AuthorPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorPartition::Unknown => write!(f, "UNKNOWN"),
            AuthorPartition::User(id) => write!(f, "USER#{id}"),
            AuthorPartition::Team(id) => write!(f, "TEAM#{id}"),
        }
    }
}

impl FromStr for AuthorPartition {
    type Err = crate::common::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(id) = s.strip_prefix("USER#") {
            Ok(AuthorPartition::User(id.to_string()))
        } else if let Some(id) = s.strip_prefix("TEAM#") {
            Ok(AuthorPartition::Team(id.to_string()))
        } else if s == "UNKNOWN" {
            Ok(AuthorPartition::Unknown)
        } else {
            Err(crate::common::Error::InvalidPartitionKey(format!(
                "invalid author partition: {s}"
            )))
        }
    }
}

impl From<UserPartition> for AuthorPartition {
    fn from(u: UserPartition) -> Self {
        AuthorPartition::User(u.0)
    }
}

impl From<TeamPartition> for AuthorPartition {
    fn from(t: TeamPartition) -> Self {
        AuthorPartition::Team(t.0)
    }
}

impl From<Partition> for AuthorPartition {
    fn from(p: Partition) -> Self {
        match p {
            Partition::User(id) => AuthorPartition::User(id),
            Partition::Team(id) => AuthorPartition::Team(id),
            _ => AuthorPartition::Unknown,
        }
    }
}
