use crate::common::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(
    Debug,
    Clone,
    Default,
    DynamoEnum,
    Eq,
    PartialEq,
    Translate,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SpaceStatus {
    #[default]
    #[translate(en = "Designing", ko = "설계중")]
    Designing,
    #[translate(en = "Open", ko = "모집중")]
    Open,
    #[translate(en = "Ongoing", ko = "진행중")]
    Ongoing,
    #[translate(en = "Processing", ko = "집계중")]
    Processing,
    #[translate(en = "Completed", ko = "종료")]
    Finished,
}

impl Serialize for SpaceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            SpaceStatus::Designing => "Designing",
            SpaceStatus::Open => "Open",
            SpaceStatus::Ongoing => "Ongoing",
            SpaceStatus::Processing => "Processing",
            SpaceStatus::Finished => "Finished",
        })
    }
}

impl<'de> Deserialize<'de> for SpaceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.as_str() {
            "Designing" | "DESIGNING" | "Waiting" | "WAITING" => Ok(SpaceStatus::Designing),
            "Open" | "OPEN" | "InProgress" | "IN_PROGRESS" => Ok(SpaceStatus::Open),
            "Ongoing" | "ONGOING" | "Started" | "STARTED" => Ok(SpaceStatus::Ongoing),
            "Processing" | "PROCESSING" => Ok(SpaceStatus::Processing),
            "Finished" | "FINISHED" => Ok(SpaceStatus::Finished),
            _ => Err(de::Error::custom(format!("Invalid SpaceStatus: {}", value))),
        }
    }
}
