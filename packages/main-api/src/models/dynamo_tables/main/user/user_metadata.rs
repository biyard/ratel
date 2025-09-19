use super::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, schemars::JsonSchema,
)]
#[serde(untagged)]
pub enum UserMetadata {
    User(User),
    UserPrincipal(UserPrincipal),
    UserEvmAddress(UserEvmAddress),
    UserReferralCode(UserReferralCode),
    UserPhoneNumber(UserPhoneNumber),
    UserTelegram(UserTelegram),
}
