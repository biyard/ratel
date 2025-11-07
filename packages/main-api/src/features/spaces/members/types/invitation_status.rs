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
    JsonSchema_repr,
    DynamoEnum,
    OperationIo,
)]
#[repr(u8)]
pub enum InvitationStatus {
    #[default]
    #[schemars(title = "Pending", description = "Invitation not yet sent")]
    Pending = 1,
    #[schemars(title = "Invited", description = "Invitation email sent to user")]
    Invited = 2,
    #[schemars(title = "Accepted", description = "User accepted the invitation")]
    Accepted = 3,
    #[schemars(title = "Declined", description = "User declined the invitation")]
    Declined = 4,
}
