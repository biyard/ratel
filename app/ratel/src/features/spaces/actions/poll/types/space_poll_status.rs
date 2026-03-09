use crate::features::spaces::actions::poll::*;

use crate::features::spaces::actions::poll::macros::DynamoEnum;

#[derive(Debug, Clone, Default, DynamoEnum, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PollStatus {
    #[default]
    NotStarted = 1,
    InProgress = 2,
    Finish = 3,
}
