use super::MembershipTier;
use serde;
use super::super::*;

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TransactionType {
    #[default]
    None,

    PurchaseMembership(MembershipTier),
}

impl TryInto<MembershipTier> for TransactionType {
    fn try_into(self) -> Result<MembershipTier> {
        match self {
            TransactionType::PurchaseMembership(tier) => Ok(tier),
            _ => Err(crate::features::social::types::SocialError::InvalidMembershipTier.into()),
        }
    }

    type Error = super::super::Error;
}
