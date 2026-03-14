use crate::features::spaces::*;
use crate::common::utils::time;
use serde::{Deserialize, Serialize};
use crate::features::auth::models::user::User;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    DynamoEnum,
)]
#[repr(u8)]
pub enum InvitationStatus {
    #[default]
    Pending = 1,
    Invited = 2,
    Accepted = 3,
    Declined = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct SpaceInvitationMember {
    #[cfg_attr(
        feature = "server",
        dynamo(
            index = "gsi3",
            name = "find_space_invitations_by_status",
            prefix = "SIM",
            pk
        )
    )]
    pub pk: Partition,
    #[cfg_attr(feature = "server", dynamo(index = "gsi1", sk))]
    pub sk: EntityType,
    #[cfg_attr(
        feature = "server",
        dynamo(
            prefix = "SPACE_INVITATION",
            name = "find_by_user_pk",
            index = "gsi1",
            pk
        )
    )]
    #[cfg_attr(
        feature = "server",
        dynamo(
            prefix = "SPACE_INVITATION",
            name = "find_user_invitations_by_status",
            index = "gsi2",
            pk
        )
    )]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", order = 1, sk))]
    #[cfg_attr(feature = "server", dynamo(index = "gsi3", sk))]
    pub status: InvitationStatus,
    #[serde(default)]
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", order = 2, sk))]
    pub created_at: i64,
}

impl SpaceInvitationMember {
    pub fn new(
        space_pk: Partition,
        User {
            pk,
            display_name,
            profile_url,
            username,
            email,
            ..
        }: User,
    ) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpaceInvitationMember(pk.to_string()),
            user_pk: pk,
            display_name,
            profile_url,
            username,
            email,
            status: InvitationStatus::Pending,
            created_at: time::get_now_timestamp_millis(),
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceInvitationMember(user_pk.to_string()),
        )
    }
}
