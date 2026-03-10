use crate::common::macros::DynamoEnum;
use crate::features::spaces::pages::apps::*;
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, DynamoEnum, Eq, PartialEq, Translate,
)]
pub enum SpaceAppType {
    #[default]
    #[translate(en = "General", ko = "스페이스 설정")]
    General,
    #[translate(en = "File", ko = "파일")]
    File,
    #[translate(en = "Panels", ko = "패널")]
    Panels,
    #[translate(en = "Incentive Pool", ko = "인센티브 풀")]
    IncentivePool,
}

impl SpaceAppType {
    pub fn is_default(&self) -> bool {
        matches!(self, SpaceAppType::General | SpaceAppType::File)
    }

    pub fn settings_path(&self, space_id: &SpacePartition) -> String {
        match self {
            SpaceAppType::General => format!("/spaces/{space_id}/apps/general"),
            SpaceAppType::File => format!("/spaces/{space_id}/apps/file"),
            SpaceAppType::Panels => format!("/spaces/{space_id}/apps/panels"),
            SpaceAppType::IncentivePool => format!("/spaces/{space_id}/apps/incentive-pool"),
        }
    }
}
