use crate::features::spaces::pages::actions::actions::poll::*;

use crate::features::spaces::pages::actions::actions::poll::macros::DynamoEnum;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, DynamoEnum, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum PollStatus {
    #[default]
    NotStarted = 1,
    InProgress = 2,
    Finish = 3,
}
