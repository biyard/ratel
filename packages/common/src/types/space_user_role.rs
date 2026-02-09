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
    PartialEq,
    Eq,
)]
pub enum SpaceUserRole {
    #[default]
    Viewer,
    Participant,
    Candidate,
    Creator,
}
