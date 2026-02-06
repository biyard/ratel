use crate::*;
use chrono::Datelike;

#[derive(
    Debug,
    Default,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    OperationIo,
    DynamoEnum,
    Eq,
    PartialEq,
)]
pub enum RewardCondition {
    #[default]
    None,
    // 전체 Reward 최대 횟 수 제한
    MaxClaims(i64),
    // 전체 Reward 최대 포인트 제한
    MaxPoints(i64),
    // 유저 당 Reward 수 제한
    MaxUserClaims(i64),
    // 유저 당 포인트 제한
    MaxUserPoints(i64),
}
