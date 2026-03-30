use crate::common::types::*;
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub struct SpaceAdmin {
    pub pk: Partition,   // SPACE#{space_id}
    pub sk: EntityType,  // SPACE_ADMIN#{user_pk}

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    pub display_name: String,
    pub username: String,
    pub profile_url: String,

    #[dynamo(prefix = "SA", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition, // USER#{user_id}
}

impl SpaceAdmin {
    pub fn new(
        space_pk: Partition,
        user_pk: Partition,
        display_name: String,
        username: String,
        profile_url: String,
    ) -> Self {
        let created_at = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: space_pk,
            sk: EntityType::SpaceAdmin(user_pk.to_string()),
            created_at,
            display_name,
            username,
            profile_url,
            user_pk,
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceAdmin(user_pk.to_string()),
        )
    }
}
