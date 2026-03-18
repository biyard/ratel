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
pub enum SpaceStatus {
    #[default]
    #[translate(en = "Processing", ko = "집계중")]
    Waiting,
    #[translate(en = "Open", ko = "모집중")]
    InProgress,
    #[translate(en = "Ongoing", ko = "진행중")]
    Started,
    #[translate(en = "Completed", ko = "종료")]
    Finished,
}
