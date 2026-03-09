use crate::features::spaces::*;
use serde::{Deserialize, Serialize};
use crate::features::auth::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct SpacePanelParticipant {
    pub pk: Partition,
    #[cfg_attr(feature = "server", dynamo(index = "gsi1", sk))]
    pub sk: EntityType,
    #[cfg_attr(
        feature = "server",
        dynamo(index = "gsi2", name = "find_by_space_and_user", pk)
    )]
    pub space_pk: Partition,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)
    )]
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", sk))]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl SpacePanelParticipant {
    pub fn new(
        space_pk: Partition,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let (space_pk, sk) = Self::keys(&space_pk, &pk);

        Self {
            pk: space_pk.clone(),
            sk,
            space_pk,
            user_pk: pk,
            display_name,
            profile_url,
            username,
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpacePanelParticipant(user_pk.to_string()),
        )
    }
}
