use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    DynamoEnum,
    JsonSchema_repr,
    OperationIo,
)]
#[repr(u8)]
pub enum ParticipantStatus {
    #[default]
    Invited = 1,
    Participating = 2,
}
