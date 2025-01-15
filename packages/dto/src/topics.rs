use std::{fmt::Display, str::FromStr};

#[cfg(feature = "server")]
use by_axum::aide;
use chrono::Datelike;
use num_format::{Locale, ToFormattedString};
#[cfg(feature = "server")]
use schemars::JsonSchema;

use crate::CommonQueryResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TopicSummary {
    pub id: String,
    pub r#type: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
    pub author: String,

    pub title: String,
    // Legislation summary
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
    pub volume: u64,
    pub status: TopicStatus,
    pub weekly_volume: u64,
    pub weekly_replies: u64,
    pub weekly_votes: u64,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TopicQuery {
    pub size: usize,
    pub bookmark: Option<String>,
    pub status: Option<TopicStatus>,
}
impl_display!(TopicQuery);

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct TopicClient {
    pub endpoint: String,
}

impl TopicSummary {
    pub fn get_client(endpoint: String) -> TopicClient {
        TopicClient { endpoint }
    }
}

impl TopicClient {
    pub async fn query(
        &self,
        params: TopicQuery,
    ) -> crate::Result<CommonQueryResponse<TopicSummary>> {
        let endpoint = format!("{}/v1/topics?{params}", self.endpoint);

        rest_api::get(&endpoint).await
    }

    pub async fn get(&self, id: &str) -> crate::Result<Topic> {
        let endpoint = format!("{}/v1/topics/{id}", self.endpoint);

        rest_api::get(&endpoint).await
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
pub struct TopicDetails {
    pub voting_trends: Vec<VoteData>,
    pub legislation_link: String,
    pub solutions: String,
    pub discussions: Vec<String>,
    pub additional_resources: Vec<AdditionalResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Topic {
    pub topic: TopicSummary,
    pub my_info: MyInfo,
    pub details: TopicDetails,
    pub comments: Vec<Commment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MyInfo {
    // If my_commitment is 1, it shows 0.01 ETH in the UI
    pub my_commitment: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Commment {
    pub profile_url: String,
    pub choice: Vote,
    pub nickname: String,
    pub content: String,
    pub created_at: u64,
    pub likes: u64,
    pub is_liked: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TopicCreateRequest {
    pub title: String,
    pub content: String,
    pub legislation_link: String,
    pub solutions: String,
    pub discussions: Vec<String>,
    pub additional_resources: Vec<AdditionalResource>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum TopicActionRequest {
    Create(TopicCreateRequest),
}

pub type CommentId = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum TopicByIdActionRequest {
    Vote(Vote),
    Comment(String),
    Like { comment_id: String, like: bool },
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
