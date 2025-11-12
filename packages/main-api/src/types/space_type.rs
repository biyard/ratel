use crate::{
    features::spaces::{panels::SpacePanels, polls::Poll},
    *,
};
use aws_sdk_dynamodb::types::TransactWriteItem;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    schemars::JsonSchema_repr,
)]
#[repr(u8)]
pub enum SpaceType {
    #[default]
    Legislation = 1,
    Poll = 2,
    Deliberation = 3,
    Nft = 4,
    Commitee = 5,
    SprintLeague = 6,
    Notice = 7,
    Dagit = 8,
}

impl SpaceType {
    pub fn create_hook(&self, space_pk: &Partition) -> Result<Vec<TransactWriteItem>> {
        let txs = match self {
            SpaceType::Poll => {
                let poll: Poll = space_pk.clone().try_into()?;

                vec![poll.create_transact_write_item()]
            }
            SpaceType::Deliberation => {
                let panel = SpacePanels::new(space_pk.clone(), 0, vec![]);

                vec![panel.create_transact_write_item()]
            }
            _ => {
                vec![]
            }
        };

        Ok(txs)
    }
}
