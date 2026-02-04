use crate::types::*;
use crate::utils::time::get_now_timestamp_millis;
use bdk::prelude::*;

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
pub struct SpaceDaoIncentiveUser {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi2", sk)]
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
    #[dynamo(
        prefix = "SPACE_DAO_INCENTIVE",
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
    pub incentive_distributed: bool,
}

impl SpaceDaoIncentiveUser {
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
        let sk = EntityType::SpaceDaoIncentive(user_pk.to_string());

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
            incentive_distributed: false,
        }
    }
}
