use crate::common::{types::*, utils::time::get_now_timestamp_millis, *};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct Reward {
    pub pk: Partition,
    pub sk: RewardUserBehavior,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(name = "find_by_action", prefix = "ACTION", index = "gsi1", pk)]
    pub action: RewardAction,

    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

#[cfg(feature = "server")]
impl Reward {
    pub fn new(
        user_behavior: RewardUserBehavior,
        point: i64,
        period: RewardPeriod,
        condition: RewardCondition,
    ) -> Self {
        let now = get_now_timestamp_millis();

        Self {
            pk: Partition::Reward,
            action: user_behavior.action(),
            sk: user_behavior,
            created_at: now,
            updated_at: now,
            point,
            period,
            condition,
        }
    }
}
