use crate::types::*;
use crate::utils::time::get_now_timestamp_millis;
use bdk::prelude::*;

use crate::features::spaces::SpaceParticipant;

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
    #[dynamo(index = "gsi2", sk)]
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(
        prefix = "SPACE_DAO_SAMPLE",
        name = "find_by_space",
        index = "gsi2",
        pk
    )]
    pub space_pk: Partition,

    pub user_pk: Partition,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub evm_address: String,
    pub reward_distributed: bool,
}

impl SpaceDaoSampleUser {
    pub fn keys(space_pk: &Partition, sk: &EntityType) -> (Partition, EntityType) {
        (space_pk.clone(), sk.clone())
    }

    pub fn new(
        space_pk: Partition,
        user_pk: Partition,
        username: String,
        display_name: String,
        profile_url: String,
        evm_address: String,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let sk = EntityType::SpaceDaoSample(format!("TS#{}#{}", now, user_pk.to_string()));

        Self {
            pk: space_pk.clone(),
            space_pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk,
            username,
            display_name,
            profile_url,
            evm_address,
            reward_distributed: false,
        }
    }
}
