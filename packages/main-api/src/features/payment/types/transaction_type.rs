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

    PurchaseMembership(MembershipTier),
}

impl TryInto<MembershipTier> for TransactionType {
    fn try_into(self) -> Result<MembershipTier> {
        match self {
            TransactionType::PurchaseMembership(tier) => Ok(tier),
            _ => Err(Error::InvalidMembershipTier),
        }
    }

    type Error = crate::Error;
}
