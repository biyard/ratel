use crate::common::utils::time::get_now_timestamp_millis;
use crate::common::*;
use crate::common::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserFollow {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,

    pub user_pk: Partition,
    pub target_user_pk: Partition,
    #[serde(default)]
    pub space_id: Option<String>,
    #[serde(default)]
    pub action_id: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub profile_url: Option<String>,
}

#[cfg(feature = "server")]
impl UserFollow {
    pub fn new_follow_records(
        follower_pk: Partition,
        target_pk: Partition,
    ) -> (Self, Self) {
        Self::new_follow_records_with_space(follower_pk, target_pk, None, None, None, None)
    }

    pub fn new_follow_records_with_space(
        follower_pk: Partition,
        target_pk: Partition,
        space_id: Option<String>,
        action_id: Option<String>,
        display_name: Option<String>,
        profile_url: Option<String>,
    ) -> (Self, Self) {
        let now = get_now_timestamp_millis();
        let follower_id = follower_pk.to_string();
        let target_id = target_pk.to_string();

        let follower_record = UserFollow {
            pk: target_pk.clone(),
            sk: EntityType::Follower(follower_id.clone()),
            created_at: now,
            user_pk: follower_pk.clone(),
            target_user_pk: target_pk.clone(),
            space_id: space_id.clone(),
            action_id: action_id.clone(),
            display_name: display_name.clone(),
            profile_url: profile_url.clone(),
        };

        let following_record = UserFollow {
            pk: follower_pk.clone(),
            sk: EntityType::Following(target_id),
            created_at: now,
            user_pk: follower_pk,
            target_user_pk: target_pk,
            space_id,
            action_id,
            display_name,
            profile_url,
        };

        (follower_record, following_record)
    }

    pub fn follower_keys(target_pk: &Partition, follower_pk: &Partition) -> (Partition, EntityType) {
        let follower_id = follower_pk.to_string();
        (target_pk.clone(), EntityType::Follower(follower_id))
    }

    pub fn following_keys(
        follower_pk: &Partition,
        target_pk: &Partition,
    ) -> (Partition, EntityType) {
        let target_id = target_pk.to_string();
        (follower_pk.clone(), EntityType::Following(target_id))
    }
}
