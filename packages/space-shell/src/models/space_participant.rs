use crate::*;
use common::utils::time;
#[cfg(feature = "server")]
use names::{Generator, Name};
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
    #[cfg(feature = "server")]
    pub fn new(space_pk: Partition, user_pk: Partition) -> Self {
        let created_at = time::get_now_timestamp_millis();
        let display_name = Generator::with_naming(Name::Numbered)
            .next()
            .unwrap()
            .replace('-', " ");
        let username = display_name.replace(' ', "-").to_lowercase();

        Self {
            pk: CompositePartition(space_pk.clone(), user_pk.clone()),
            sk: EntityType::SpaceParticipant,
            created_at,
            display_name,
            username,
            profile_url: "https://metadata.ratel.foundation/ratel/default-profile.png".to_string(),
            user_type: UserType::AnonymousSpaceUser,
            space_pk,
            user_pk,
        }
    }

    pub fn new_non_anonymous(
        space_pk: Partition,
        ratel_auth::User {
            pk,
            username,
            display_name,
            profile_url,
            user_type,
            ..
        }: ratel_auth::User,
    ) -> Self {
        let created_at = time::get_now_timestamp_millis();
        let user_pk = pk;
        Self {
            pk: CompositePartition(space_pk.clone(), user_pk.clone()),
            sk: EntityType::SpaceParticipant,
            created_at,
            display_name,
            username,
            profile_url,
            user_type,
            space_pk,
            user_pk,
        }
    }

    pub fn keys(space_pk: Partition, user_pk: Partition) -> (CompositePartition, EntityType) {
        (
            CompositePartition(space_pk, user_pk),
            EntityType::SpaceParticipant,
        )
    }
}
