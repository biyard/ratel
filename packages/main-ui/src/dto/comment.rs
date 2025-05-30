use bdk::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Comment {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub profile_url: String,
    pub profile_name: String,

    pub comment: String, //html format
    pub replies: Vec<Comment>,

    pub number_of_comments: i64,
    pub number_of_likes: i64,
}
