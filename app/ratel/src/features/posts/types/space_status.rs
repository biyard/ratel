use crate::features::posts::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, serde_with::SerializeDisplay, serde_with::DeserializeFromStr, Default, DynamoEnum, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum SpaceStatus {
    #[default]
    Waiting, // Draft
    InProgress, // Published
    Started,    // Started
    Finished,   // Finished
}
