use crate::features::my_follower::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct FollowUserItem {
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
    pub description: String,
    pub is_following: bool,
}

#[derive(Debug, Clone)]
pub struct FollowUserProfile {
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
    pub description: String,
}
