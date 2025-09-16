use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Partition {
    User,
    Email,
}

impl Partition {
    pub fn key<T: Display>(self, id: T) -> String {
        format!("{}#{}", self, id)
    }
}
