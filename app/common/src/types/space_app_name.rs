use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    Translate,
    PartialEq,
    Eq,
)]
#[serde(rename_all = "snake_case")]
pub enum SpaceAppName {
    #[default]
    #[translate(ko = "전체 앱")]
    AllApps,
    #[translate(ko = "스페이스 설정")]
    General,
    #[translate(ko = "인센티브 풀")]
    IncentivePool,
}

impl SpaceAppName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllApps => "all_apps",
            Self::General => "general",
            Self::IncentivePool => "incentive_pool",
        }
    }
}

impl std::fmt::Display for SpaceAppName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for SpaceAppName {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "all_apps" => Ok(Self::AllApps),
            "general" => Ok(Self::General),
            "incentive_pool" => Ok(Self::IncentivePool),
            _ => Err(()),
        }
    }
}
