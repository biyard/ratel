use crate::common::*;

#[derive(
    Debug,
    Default,
    Clone,
    Serialize,
    Deserialize,
    DynamoEnum,
    Eq,
    PartialEq,
)]
pub enum RewardCondition {
    #[default]
    None,
    MaxClaims(i64),
    MaxPoints(i64),
    MaxUserClaims(i64),
    MaxUserPoints(i64),
}
