use crate::utils::time::get_now_timestamp_millis;
use crate::*;
use crate::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserFollow {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,

    pub user_pk: Partition,
    pub target_user_pk: Partition,
}

#[cfg(feature = "server")]
impl UserFollow {
    pub fn new_follow_records(
        follower_pk: Partition,
        target_pk: Partition,
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
        };

        let following_record = UserFollow {
            pk: follower_pk.clone(),
            sk: EntityType::Following(target_id),
            created_at: now,
            user_pk: follower_pk,
            target_user_pk: target_pk,
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
