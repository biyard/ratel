use crate::types::*;
use crate::utils::time::get_now_timestamp_millis;
use bdk::prelude::*;

use super::space_participant::SpaceParticipant;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceDaoSampleUser {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub user_pk: Partition,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub evm_address: String,
    pub reward_distributed: bool,
}

impl SpaceDaoSampleUser {
    pub fn new(space_pk: Partition, participant: SpaceParticipant, evm_address: String) -> Self {
        let now = get_now_timestamp_millis();
        let user_pk = participant.user_pk.clone();
        let sk = EntityType::SpaceDaoSample(format!(
            "TS#{}#{}",
            now,
            user_pk.to_string()
        ));

        Self {
            pk: space_pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk: participant.user_pk,
            username: participant.username,
            display_name: participant.display_name,
            profile_url: participant.profile_url,
            evm_address,
            reward_distributed: false,
        }
    }
}
