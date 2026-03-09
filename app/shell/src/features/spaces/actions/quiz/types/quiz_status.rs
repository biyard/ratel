use crate::features::spaces::actions::quiz::*;

use crate::features::spaces::actions::quiz::macros::DynamoEnum;

#[derive(Debug, Clone, Default, DynamoEnum, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum QuizStatus {
    #[default]
    NotStarted = 1,
    InProgress = 2,
    Finish = 3,
}
