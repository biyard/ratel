use crate::QueryResponse;
use by_macros::api_model;
use chrono::Datelike;
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use super::comment::*;

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[api_model(base = "/topics/v1", database = skip, iter_type=QueryResponse)]
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(summary)]
    pub created_at: u64,
    #[api_model(summary)]
    pub updated_at: u64,
    #[api_model(summary)]
    pub deleted_at: Option<u64>,
    #[api_model(summary)]
    pub author: String,

    #[api_model(summary, action = create)]
    pub title: String,
    // Legislation summary
    #[api_model(summary, action = create)]
    pub content: String,

    // The image URLs of the voting topic
    #[api_model(summary)]
    pub images: Vec<String>,
    #[serde(default)]
    #[api_model(summary)]
    pub result: Option<TopicResult>,
    #[api_model(summary)]
    pub votes: Vec<Vote>,
    #[api_model(summary)]
    pub donations: Vec<Donation>,
    // The start time of the vote
    #[api_model(summary)]
    pub started_at: i64,
    // The end time of the vote
    #[api_model(summary)]
    pub ended_at: i64,
    // The number of voters
    #[api_model(summary)]
    pub voters: u64,
    // The number of replies
    #[api_model(summary)]
    pub replies: u64,
    #[api_model(summary)]
    pub volume: u64,
    #[api_model(summary, queryable)]
    pub status: TopicStatus,
    #[api_model(summary)]
    pub weekly_volume: u64,
    #[api_model(summary)]
    pub weekly_replies: u64,
    #[api_model(summary)]
    pub weekly_votes: u64,

    pub my_info: MyInfo,

    pub voting_trends: Vec<VoteData>,
    #[api_model(action = create)]
    pub legislation_link: String,
    #[api_model(action = create)]
    pub solutions: String,
    #[api_model(action = create)]
    pub discussions: Vec<String>,
    #[api_model(action = create)]
    pub additional_resources: Vec<AdditionalResource>,

    #[api_model(action_by_id = comment, related = String)]
    pub comments: Vec<Comment>,

    // It shows the voting opinion of the signer.
    #[api_model(action_by_id = vote)]
    pub vote: Option<Vote>,
    // If signer liked this topic, it will be true. Otherwise, it will be false.
    #[api_model(action_by_id = like)]
    pub like: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum TrendTag {
    Hot,
    Warm,
    Cold,
}

impl Display for TrendTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrendTag::Hot => write!(f, "HOT"),
            TrendTag::Warm => write!(f, "WARM"),
            TrendTag::Cold => write!(f, "COLD"),
        }
    }
}

impl TopicSummary {
    pub fn number_of_yes(&self) -> u64 {
        self.votes
            .iter()
            .filter_map(|r| match r {
                Vote::Supportive(y) => Some(*y),
                _ => None,
            })
            .sum()
    }

    pub fn number_of_no(&self) -> u64 {
        self.votes
            .iter()
            .filter_map(|r| match r {
                Vote::Against(n) => Some(*n),
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
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
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
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
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
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Vote {
    Supportive(u64),
    Against(u64),
    Neutral(u64),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Donation {
    Yes(u64),
    No(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct VoteData {
    pub voted_at: i64,
    pub vote: Vote,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum FileType {
    Image,
    Video,
    Audio,
    Pdf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AdditionalResource {
    pub filename: String,
    pub extension: FileType,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MyInfo {
    // If my_commitment is 1, it shows 0.01 ETH in the UI
    pub my_commitment: u64,
}

impl TopicSummary {
    pub fn trend_tag(&self) -> TrendTag {
        if self.weekly_volume > 100 {
            TrendTag::Hot
        } else if self.weekly_volume > 50 {
            TrendTag::Warm
        } else {
            TrendTag::Cold
        }
    }

    pub fn day(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!("{:02}", start.day())
    }

    pub fn month(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        match start.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Unknown",
        }
        .to_string()
    }

    pub fn date(&self) -> String {
        format!("{}/{}", self.month(), self.day())
    }

    pub fn volume_with_commas(&self) -> String {
        self.volume.to_formatted_string(&Locale::en)
    }
}
