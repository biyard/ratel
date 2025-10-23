use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    schemars::JsonSchema,
    aide::OperationIo,
    Eq,
    PartialEq,
)]
pub enum MembershipStatus {
    #[default]
    Active,
    Expired,
    Cancelled,
    Suspended,
}
