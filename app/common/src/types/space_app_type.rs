use crate::macros::DynamoEnum;

use crate::*;
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Translate, PartialEq, Eq, DynamoEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum SpaceAppType {
    #[default]
    #[translate(ko = "스페이스 설정")]
    General,
    #[translate(ko = "인센티브 풀")]
    IncentivePool,
    #[translate(ko = "파일")]
    File,
}
