pub mod common_query_response;
pub mod error;

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct Topic {
    pub id: String,
    pub r#type: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
    pub author: String,

    pub title: String,
    pub content: String,

    // The image URLs of the voting topic
    pub images: Vec<String>,
    pub results: Vec<Vote>,
    pub donations: Vec<Donation>,
    // The start time of the vote
    pub started_at: u64,
    // The end time of the vote
    pub ended_at: u64,
    // The number of voters
    pub voters: u64,
    // The number of replies
    pub replies: u64,
    pub status: TopicStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum TopicStatus {
    Finished,
    Ongoing,
    Scheduled,
    Cancelled,
    Draft,
}

impl Default for TopicStatus {
    fn default() -> Self {
        TopicStatus::Draft
    }
}

impl Display for TopicStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicStatus::Finished => write!(f, "finished"),
            TopicStatus::Ongoing => write!(f, "ongoing"),
            TopicStatus::Scheduled => write!(f, "scheduled"),
            TopicStatus::Cancelled => write!(f, "cancelled"),
            TopicStatus::Draft => write!(f, "draft"),
        }
    }
}

impl FromStr for TopicStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "finished" => Ok(TopicStatus::Finished),
            "ongoing" => Ok(TopicStatus::Ongoing),
            "scheduled" => Ok(TopicStatus::Scheduled),
            "cancelled" => Ok(TopicStatus::Cancelled),
            "draft" => Ok(TopicStatus::Draft),
            _ => Err(format!("unknown topic status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Vote {
    Yes(u64),
    No(u64),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Donation {
    Yes(u64),
    No(u64),
}
