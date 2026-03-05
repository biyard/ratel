use common::utils::time::get_now_timestamp_millis;

use crate::macros::DynamoEntity;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceSubscriptionUser {
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
}

#[cfg(feature = "server")]
impl SpaceSubscriptionUser {
    pub fn new(space_pk: SpacePartition, user: &ratel_auth::User) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceSubscriptionUser(user.pk.to_string());

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk: user.pk.clone(),
            display_name: user.display_name.clone(),
            profile_url: user.profile_url.clone(),
            username: user.username.clone(),
            user_type: user.user_type.clone(),
        }
    }

    pub fn keys(space_pk: &SpacePartition, user_pk: &Partition) -> (Partition, EntityType) {
        let pk: Partition = space_pk.clone().into();
        let sk = EntityType::SpaceSubscriptionUser(user_pk.to_string());
        (pk, sk)
    }
}
