use crate::*;
use ratel_auth::models::user::UserType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct SpaceParticipant {
    pub pk: CompositePartition,
    pub sk: EntityType,
    #[cfg_attr(feature = "server", dynamo(index = "gsi1", sk))]
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", sk))]
    pub created_at: i64,
    pub display_name: String,
    #[cfg_attr(feature = "server", dynamo(index = "gsi3", sk))]
    pub username: String,
    pub profile_url: String,
    pub user_type: UserType,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "SP", name = "find_by_space", index = "gsi2", pk)
    )]
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "SP", name = "search_users_by_space", index = "gsi3", pk)
    )]
    pub space_pk: Partition,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "SP", name = "find_by_user", index = "gsi1", pk)
    )]
    pub user_pk: Partition,
}

impl SpaceParticipant {
    pub fn keys(space_pk: Partition, user_pk: Partition) -> (CompositePartition, EntityType) {
        (
            CompositePartition(space_pk, user_pk),
            EntityType::SpaceParticipant,
        )
    }
}
