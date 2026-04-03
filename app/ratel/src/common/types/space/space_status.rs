use crate::common::*;

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
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
