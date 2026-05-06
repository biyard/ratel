use serde;
use super::super::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum MembershipTier {
    #[default]
    Free,
    Pro,
    Max,
    Vip,
    Enterprise(String),
}
