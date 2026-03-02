use crate::serde_with::{DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;

use crate::*;
#[derive(
    Debug,
    Clone,
    Copy,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    Translate,
    PartialEq,
    Eq,
    DynamoEnum,
)]
pub enum SpaceAppType {
    #[default]
    #[translate(en = "General", ko = "스페이스 설정")]
    General,
    #[translate(en = "Incentive Pool", ko = "인센티브 풀")]
    IncentivePool,
}
