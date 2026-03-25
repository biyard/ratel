use crate::common::{utils::time::get_now_timestamp_millis, *};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct PendingReward {
    pub pk: String,
    pub sk: String,

    pub created_at: i64,
    pub target_pk: String,
    pub owner_pk: String,
    pub space_pk: String,
    pub reward_key: String,
    pub amount: i64,
    pub description: String,
    pub status: String,
    pub retry_count: i64,
}

#[cfg(feature = "server")]
impl PendingReward {
    pub fn new(
        target_pk: &Partition,
        space_pk: &Partition,
        reward_key: &str,
        amount: i64,
        description: &str,
        owner_pk: Option<&Partition>,
    ) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: "PENDING_REWARD".to_string(),
            sk: format!("PENDING_REWARD#{}#{}#{}", now, target_pk, reward_key),
            created_at: now,
            target_pk: target_pk.to_string(),
            owner_pk: owner_pk.map(|p| p.to_string()).unwrap_or_default(),
            space_pk: space_pk.to_string(),
            reward_key: reward_key.to_string(),
            amount,
            description: description.to_string(),
            status: "pending".to_string(),
            retry_count: 0,
        }
    }
}
