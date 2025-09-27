use bdk::prelude::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

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

    Team(String),
}
