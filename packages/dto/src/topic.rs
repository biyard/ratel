#![allow(unused, dead_code)]
use crate::*;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;

use chrono::Datelike;
use dioxus_translate::Translate;
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use super::comment::*;

#[cfg(feature = "server")]
use schemars::JsonSchema;

#[api_model(base = "/v1/topics", table = topics, iter_type=QueryResponse)]
pub struct Topic {
    #[api_model(summary, primary_key)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: u64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: u64,
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
    pub result: TopicResult,
    // The start time of the vote
    #[api_model(summary)]
    pub started_at: i64,
    // The end time of the vote
    #[api_model(summary)]
    pub ended_at: i64,
    #[api_model(summary, queryable)]
    pub status: TopicStatus,
    // pub voting_trends: Vec<VoteData>,
    #[api_model(action = create)]
    pub legislation_link: String,
    #[api_model(action = create)]
    pub solutions: String,
    #[api_model(action = create, type = JSONB)]
    pub discussions: Vec<String>,
    #[api_model(action = create, type = JSONB)]
    pub additional_resources: Vec<AdditionalResource>,

    #[api_model(summary, one_to_many = votes, foreign_key = topic_id, aggregator = sum(amount))]
    pub volume: i64,

    #[api_model(summary, one_to_many = comments, foreign_key = topic_id, aggregator = count)]
    pub replies: i64,

    // User-specific information
    #[api_model(many_to_many = votes, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = topic_id, unique)]
    pub vote: Vec<Vote>,

    #[api_model(many_to_many = topic_likes, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = topic_id, aggregator = exist)]
    pub like: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum TrendTag {
    Hot,
    Warm,
    Cold,
}

impl TopicSummary {
    pub fn number_of_yes(&self) -> u64 {
        10
        // self.votes
        //     .iter()
        //     .filter_map(|r| match r {
        //         VoteResult::Supportive(y) => Some(*y),
        //         _ => None,
        //     })
        //     .sum()
    }

    pub fn number_of_no(&self) -> u64 {
        20
        // self.votes
        //     .iter()
        //     .filter_map(|r| match r {
        //         VoteResult::Against(n) => Some(*n),
        //         _ => None,
        //     })
        //     .sum()
    }

    pub fn donations(&self) -> u64 {
        0
        // self.donations
        //     .iter()
        //     .map(|r| match r {
        //         Donation::Yes(y) => y,
        //         Donation::No(n) => n,
        //     })
        //     .sum::<u64>()
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

#[derive(
    Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Copy, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum TopicResult {
    #[default]
    #[translate(en = "", ko = "")]
    None = 0,
    #[translate(en = "Accepted", ko = "통과")]
    Accepted = 1,
    #[translate(en = "Rejected", ko = "거절")]
    Rejected = 2,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Copy, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum TopicStatus {
    #[default]
    Draft = 0,
    Scheduled = 1,
    Ongoing = 2,
    Finished = 3,
    Cancelled = 4,
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

impl TopicSummary {
    pub fn trend_tag(&self) -> TrendTag {
        TrendTag::Hot
        //     if self.weekly_volume > 100 {
        //     TrendTag::Hot
        // } else if self.weekly_volume > 50 {
        //     TrendTag::Warm
        // } else {
        //     TrendTag::Cold
        // }
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
