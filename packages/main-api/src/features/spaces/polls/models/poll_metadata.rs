use bdk::prelude::*;

use crate::{
    features::spaces::polls::{Poll, PollQuestion},
    types::{EntityType, Partition},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PollMetadata {
    Poll(Poll),
    PollQuestion(PollQuestion),
}

impl PollMetadata {
    pub async fn query_all(
        cli: &aws_sdk_dynamodb::Client,
        pk: &Partition,
    ) -> crate::Result<Vec<PollMetadata>> {
        let mut prefix = EntityType::SpacePoll(String::default()).to_string();
        prefix.retain(|c| c != '#');

        let res = PollMetadata::query_begins_with_sk(cli, pk, prefix).await?;
        Ok(res)
    }
}
