use crate::features::spaces::rewards::{
    RewardKey, RewardPeriod, SpaceReward, TimeKey, UserRewardHistoryKey,
};
use crate::services::biyard::Biyard;
use crate::types::*;
use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, JsonSchema, OperationIo)]
pub struct UserRewardHistory {
    pub pk: CompositePartition,
    pub sk: UserRewardHistoryKey,

    pub point: i64,
    pub created_at: i64,

    pub transaction_id: Option<String>,
    pub month: Option<String>, // e.g., "2024-06"
}

impl UserRewardHistory {
    pub fn new(target_pk: Partition, space_reward: SpaceReward) -> Self {
        let now = time::get_now_timestamp_millis();
        let time_key = space_reward.period.to_time_key(now);
        let amount = space_reward.get_amount();

        let (pk, sk) = Self::key(target_pk, space_reward, time_key);

        Self {
            pk,
            sk,
            point: amount,
            created_at: now,
            ..Default::default()
        }
    }

    pub fn key(
        target_pk: Partition,
        space_reward: SpaceReward,
        time_key: TimeKey,
    ) -> (CompositePartition, UserRewardHistoryKey) {
        let pk = CompositePartition(target_pk, Partition::Reward);
        let sk = UserRewardHistoryKey(space_reward.sk, time_key);
        (pk, sk)
    }

    pub fn set_transaction(&mut self, transaction_id: String, month: String) -> &mut Self {
        self.transaction_id = Some(transaction_id);
        self.month = Some(month);
        self
    }
}
