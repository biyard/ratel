use crate::{types::*, utils::time::get_now_timestamp_millis, *};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct UserRewardHistory {
    pub pk: CompositePartition,
    pub sk: UserRewardHistoryKey,

    pub point: i64,
    pub created_at: i64,

    pub transaction_id: Option<String>,
    pub month: Option<String>,
}

#[cfg(feature = "server")]
impl UserRewardHistory {
    pub fn from_params(
        target_pk: Partition,
        reward_key: RewardKey,
        period: &RewardPeriod,
        amount: i64,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let time_key = period.to_time_key(now);
        let pk = CompositePartition(target_pk, Partition::Reward);
        let sk = UserRewardHistoryKey(reward_key, time_key);

        Self {
            pk,
            sk,
            point: amount,
            created_at: now,
            ..Default::default()
        }
    }

    pub fn set_transaction(&mut self, transaction_id: String, month: String) -> &mut Self {
        self.transaction_id = Some(transaction_id);
        self.month = Some(month);
        self
    }
}
