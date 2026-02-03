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
pub struct SpaceDaoSelectedUser {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi2", sk)]
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
    #[dynamo(
        prefix = "SPACE_DAO_SELECTED",
        name = "find_by_space",
        index = "gsi2",
        pk
    )]
    #[serde(default)]
    pub space_pk: Partition,

    #[serde(default)]
    pub user_pk: Partition,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub profile_url: String,
    #[serde(default)]
    pub evm_address: String,
    #[serde(default)]
    pub reward_distributed: bool,
}

impl SpaceDaoSelectedUser {
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
        let sk = EntityType::SpaceDaoSelected(user_pk.to_string());

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
