use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::actions::follow::macros::DynamoEntity;
use crate::features::spaces::pages::actions::actions::follow::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceFollowUser {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub user_pk: Partition,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub profile_url: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub user_type: UserType,
    #[serde(default)]
    pub description: String,
}

#[cfg(feature = "server")]
impl SpaceFollowUser {
    pub fn new(
        space_pk: SpacePartition,
        user_pk: Partition,
        display_name: String,
        profile_url: String,
        username: String,
        user_type: UserType,
        description: String,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceSubscriptionUser(user_pk.to_string());

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk,
            display_name,
            profile_url,
            username,
            user_type,
            description,
        }
    }

    pub fn keys(space_pk: &SpacePartition, user_pk: &Partition) -> (Partition, EntityType) {
        let pk: Partition = space_pk.clone().into();
        let sk = EntityType::SpaceSubscriptionUser(user_pk.to_string());
        (pk, sk)
    }
}
