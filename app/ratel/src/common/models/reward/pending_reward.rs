use crate::common::{
    types::{PendingRewardKey, PendingRewardStatus, RewardKey},
    utils::time::get_now_timestamp_millis,
    *,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct PendingReward {
    pub pk: Partition,
    pub sk: PendingRewardKey,

    pub created_at: i64,

    pub target_pk: Partition,
    #[serde(default)]
    pub owner_pk: Option<Partition>,
    pub space_pk: Partition,
    pub reward_key: RewardKey,
    pub amount: i64,
    pub description: String,

    #[dynamo(prefix = "PR_STATUS", name = "find_by_status", index = "gsi1", pk)]
    pub status: PendingRewardStatus,
    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    #[serde(default)]
    pub updated_at: i64,

    pub retry_count: i64,
    #[serde(default)]
    pub last_error: String,
}

#[cfg(feature = "server")]
impl PendingReward {
    pub fn new(
        target_pk: &Partition,
        space_pk: &Partition,
        reward_key: &RewardKey,
        amount: i64,
        description: &str,
        owner_pk: Option<&Partition>,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let sk = PendingRewardKey {
            created_at: now,
            target_pk: target_pk.clone(),
            reward_key: reward_key.clone(),
        };
        Self {
            pk: Partition::PendingReward,
            sk,
            created_at: now,
            target_pk: target_pk.clone(),
            owner_pk: owner_pk.cloned(),
            space_pk: space_pk.clone(),
            reward_key: reward_key.clone(),
            amount,
            description: description.to_string(),
            status: PendingRewardStatus::Pending,
            updated_at: now,
            retry_count: 0,
            last_error: String::new(),
        }
    }
}
