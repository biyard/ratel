use bdk::prelude::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

use serde::{Deserialize, Deserializer, de};
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    PartialEq,
    Eq,
)]
#[dynamo_enum(error = "crate::Error2")]
pub enum Partition {
    #[default]
    None,

    User(String),
    Email(String),
    Feed(String),
    PostLike(String),
    Session(String),

    // Spaces
    Space(String),
    DeliberationSpace(String),
    PollSpace(String),
    SurveySpace(String),

    Discussion(String),
    DiscussionUser(String),
    Survey(String),
    SurveyResponse(String),

    Team(String),
}

pub fn path_param_string_to_partition<'de, D>(deserializer: D) -> Result<Partition, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let url_decoded = percent_encoding::percent_decode_str(&s)
        .decode_utf8()
        .map_err(|e| de::Error::custom(format!("Invalid percent-encoding: {}", e)))?;
    let url_decoded = url_decoded.into_owned();

    Ok(Partition::from_str(&url_decoded)
        .map_err(|e| de::Error::custom(format!("Invalid Partition: {}", e)))?)
}
