use crate::{features::membership::MembershipTier, *};

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
)]
pub enum TransactionType {
    #[default]
    None,

    PurchaseMembership(String),
}
