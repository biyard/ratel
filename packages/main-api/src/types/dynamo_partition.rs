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
    aide::OperationIo,
)]
#[dynamo_enum(error = "crate::Error2")]
pub enum Partition {
    #[default]
    None,

    User(String),
    Email(String),
    Feed(String),
    PostReply(String), // POST_REPLY#{{post_pk}}
    PostLike(String),
    Session(String),

    // Spaces
    Space(String),
    DeliberationSpace(String),
    SurveySpace(String),

    Discussion(String),
    DiscussionUser(String),
    Survey(String),
    SurveyResponse(String),

    // Poll Space
    // PollSpace(String),
    PollSpaceResponse(String), // user_pk

    // Sprint League Space
    SprintLeagueVote(String), // user_pk

    Team(String),

    Promotion(String),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;
    use serde::de::value::{Error as ValueError, StringDeserializer};

    #[test]
    fn test_path_param_valid_poll_space() {
        let deserializer: StringDeserializer<ValueError> =
            String::from("FEED%23abc123").into_deserializer();
        let result = path_param_string_to_partition(deserializer);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_param_invalid_encoding() {
        let deserializer: StringDeserializer<ValueError> =
            String::from("FEED%ZZ").into_deserializer();
        let result = path_param_string_to_partition(deserializer);
        assert!(result.is_err());
    }
}
