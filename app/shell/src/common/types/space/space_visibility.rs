use crate::common::*;

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SpaceVisibility {
    #[default]
    Private, // Private space does mean that it only allows invited members or team members to access.
    Public,
    Team(String), // Use Private or Public instead. Team visibility is no longer supported.
}
