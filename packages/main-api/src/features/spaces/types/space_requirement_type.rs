use crate::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
)]
pub enum SpaceRequirementType {
    #[default]
    None,

    // PrePoll will be excuted right after participating.
    // If PrePoll exists, viewers and participants can't see any contents before finishing the PrePoll.
    PrePoll,
}
