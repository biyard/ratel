use crate::features::spaces::rewards::{RewardHistoryType, RewardPeriod, RewardType, SpaceReward};
use crate::services::biyard::Biyard;
use crate::types::*;
use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, JsonSchema, OperationIo)]
/// UserReward: 유저가 획득한 리워드 기록 ( 중복 지급 제한 )
///
/// Key Structure:
/// - PK: USER#{user_pk}##REWARD
/// - SK: REWARD_TYPE#{time_key}
///
/// time_key는 period에 따라 달라짐:
/// - Once: "ONCE"
/// - Daily: "20251201"
/// - Weekly: "2025W48"
/// - Monthly: "202512"
/// - Yearly: "2025"
/// - Unlimited: "1733850123456" (timestamp)
///
/// Examples:
/// - PK: USER#{USER_ID}##REWARD_HISTORY
/// - SK: SPACE_POLL#{POLL_UUID}#Respond#TIMESTAMP
///
/// - PK: USER#{USER_ID}##REWARD_HISTORY
/// - SK: SPACE_BOARD#{BOARD_UUID}#Respond#TIMESTAMP

pub struct UserRewardHistory {
    pub pk: CompositePartition, // USER#{user_pk}##REWARD_HISTORY
    pub sk: RewardHistoryType,  // Feature#{feature_key}#{reward_type}#{time_key}

    pub point: i64,
    pub created_at: i64,

    pub transaction_id: Option<String>, // Biyard Service Transaction ID
    pub month: Option<String>,          // e.g., "2024-06"
}

impl UserRewardHistory {
    pub fn new(user_pk: UserPartition, space_reward: SpaceReward) -> Self {
        let now = time::get_now_timestamp_millis();
        let time_key = space_reward.period.to_time_key(now);
        let (pk, sk) = Self::keys(
            user_pk,
            space_reward.get_space_pk(),
            space_reward.sk,
            time_key,
        );

        Self {
            pk,
            sk,
            point: space_reward.point,
            created_at: now,
            ..Default::default()
        }
    }
    pub fn set_transaction(&mut self, transaction_id: String, month: String) -> &mut Self {
        self.transaction_id = Some(transaction_id);
        self.month = Some(month);
        self
    }
    pub fn keys(
        user_pk: UserPartition,
        space_pk: SpacePartition,
        reward_type: RewardType,
        time_key: String,
    ) -> (CompositePartition, RewardHistoryType) {
        let user_reward_history: UserRewardHistoryPartition = UserRewardHistoryPartition(user_pk.0);

        let reward_history_type = RewardHistoryType(reward_type, time_key);
        (
            CompositePartition(user_reward_history.into(), space_pk.into()),
            reward_history_type,
        )
    }
}
