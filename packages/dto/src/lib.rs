pub mod error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingTopicResponse {
    pub id: String,
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
