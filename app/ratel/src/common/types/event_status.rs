use crate::common::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, DynamoEnum)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum EventStatus {
    #[default]
    Requested,
    Failed,
    Completed,
}
