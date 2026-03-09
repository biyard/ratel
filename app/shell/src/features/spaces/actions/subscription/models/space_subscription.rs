use common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::actions::subscription::*;

use crate::features::spaces::actions::subscription::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceSubscription {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl SpaceSubscription {
    pub fn new(space_pk: SpacePartition) -> Self {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceSubscription;
        let now = get_now_timestamp_millis();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_edit(user_role: &SpaceUserRole) -> crate::features::spaces::actions::subscription::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(crate::features::spaces::actions::subscription::Error::NoPermission),
        }
    }
}
