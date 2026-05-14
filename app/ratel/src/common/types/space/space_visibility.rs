use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum SpaceVisibility {
    #[default]
    Private, // Private space does mean that it only allows invited members or team members to access.
    Public,
    Team(String), // Use Private or Public instead. Team visibility is no longer supported.
}
