use crate::*;

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
    #[translate(ko = "대기중")]
    Waiting, // Draft
    #[translate(ko = "진행중")]
    InProgress, // Published
    // TODO: fix translate.
    #[translate(ko = "시작")]
    Started, // Started
    #[translate(ko = "종료")]
    Finished, // Finished
}
