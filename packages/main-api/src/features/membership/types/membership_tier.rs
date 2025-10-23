use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub enum MembershipTier {
    #[default]
    Free,
    Pro,
    Max,
    Vip,
    Enterprise(String),
}
