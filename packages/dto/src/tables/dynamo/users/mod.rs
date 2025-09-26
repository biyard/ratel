use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserSortKey {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "PROFILE")]
    Profile,
}

impl std::fmt::Display for UserSortKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSortKey::User => write!(f, "USER"),
            UserSortKey::Profile => write!(f, "PROFILE"),
        }
    }
}

impl From<&str> for UserSortKey {
    fn from(s: &str) -> Self {
        match s {
            "USER" => UserSortKey::User,
            "PROFILE" => UserSortKey::Profile,
            _ => panic!("Invalid sort key: {}", s),
        }
    }
}

mod user;
mod profile;

pub use user::*;
pub use profile::*;
