use crate::*;
use common::macros::DynamoEnum;
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, DynamoEnum, Eq, PartialEq, Translate,
)]
pub enum SpaceAppType {
    #[default]
    #[translate(en = "General", ko = "스페이스 설정")]
    General,
    #[translate(en = "Incentive Pool", ko = "인센티브 풀")]
    IncentivePool,
    #[translate(en = "Files", ko = "파일")]
    File,
}
