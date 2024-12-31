use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
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
    #[serde(default)]
    pub result: Option<TopicResult>,
    pub votes: Vec<Vote>,
    pub donations: Vec<Donation>,
    // The start time of the vote
    pub started_at: i64,
    // The end time of the vote
    pub ended_at: i64,
    // The number of voters
    pub voters: u64,
    // The number of replies
    pub replies: u64,
    pub status: TopicStatus,
}

impl Topic {
    pub fn number_of_yes(&self) -> u64 {
        self.votes
            .iter()
            .filter_map(|r| match r {
                Vote::Yes(y) => Some(*y),
                _ => None,
            })
            .sum()
    }

    pub fn number_of_no(&self) -> u64 {
        self.votes
            .iter()
            .filter_map(|r| match r {
                Vote::No(n) => Some(*n),
                _ => None,
            })
            .sum()
    }

    pub fn donations(&self) -> u64 {
        self.donations
            .iter()
            .map(|r| match r {
                Donation::Yes(y) => y,
                Donation::No(n) => n,
            })
            .sum::<u64>()
    }

    pub fn period(&self) -> String {
        // to "12/15 - 1/22"
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();
        let end = chrono::DateTime::from_timestamp(self.ended_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!(
            "{:02}/{:02} - {:02}/{:02}",
            start.month(),
            start.day(),
            end.month(),
            end.day()
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopicResult {
    Accepted,
    Rejected,
}

impl Default for TopicResult {
    fn default() -> Self {
        TopicResult::Rejected
    }
}

impl Display for TopicResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicResult::Accepted => write!(f, "accepted"),
            TopicResult::Rejected => write!(f, "rejected"),
        }
    }
}

impl FromStr for TopicResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accepted" => Ok(TopicResult::Accepted),
            "rejected" => Ok(TopicResult::Rejected),
            _ => Err(format!("unknown topic result: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
