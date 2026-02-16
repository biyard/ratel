use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    // DynamoEnum,
    // JsonSchema,
    // OperationIo,
    Translate,
    PartialEq,
    Eq,
)]
pub enum SpaceUserRole {
    #[default]
    #[translate(ko = "뷰어")]
    Viewer,
    #[translate(ko = "참가자")]
    Participant,
    #[translate(ko = "참가후보")]
    Candidate,
    #[translate(ko = "관리자")]
    Creator,
}
