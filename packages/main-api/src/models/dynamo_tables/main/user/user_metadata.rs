use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum UserMetadata {
    User(User),
    UserPrincipal(UserPrincipal),
    UserEvmAddress(UserEvmAddress),
    UserReferralCode(UserReferralCode),
    UserPhoneNumber(UserPhoneNumber),
    UserTelegram(UserTelegram),
}

#[derive(serde::Serialize, schemars::JsonSchema)]
pub struct UserResponse {
    pub pk: String,
    pub email: String,
    pub display_name: String,
    pub profile_url: String,
    pub content: String,
    pub user_type: u8,

    pub followers_count: i64,
    pub followings_count: i64,

    pub membership: u8,
    pub theme: u8,
    pub point: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            pk: user.pk.to_string(),
            email: user.email,
            display_name: user.display_name,
            profile_url: user.profile_url,
            content: user.html_contents,
            user_type: user.user_type as u8,
            followers_count: user.followers_count,
            followings_count: user.followings_count,
            membership: user.membership as u8,
            theme: user.theme as u8,
            point: user.points,
        }
    }
}
#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct UserDetailResponse {
    #[serde(flatten)]
    pub user: Option<UserResponse>,

    pub referral_code: Option<String>,
    pub phone_number: Option<String>,
    pub principal: Option<String>,
    pub evm_address: Option<String>,
    pub telegram: Option<String>,
}

impl From<Vec<UserMetadata>> for UserDetailResponse {
    fn from(items: Vec<UserMetadata>) -> Self {
        let mut res = Self::default();
        for item in items {
            match item {
                UserMetadata::User(user) => {
                    res.user = Some(user.into());
                }

                UserMetadata::UserReferralCode(user_referral_code) => {
                    res.referral_code = Some(user_referral_code.referral_code);
                }
                UserMetadata::UserPhoneNumber(user_phone_number) => {
                    res.phone_number = Some(user_phone_number.phone_number);
                }
                UserMetadata::UserPrincipal(user_principal) => {
                    res.principal = Some(user_principal.principal);
                }
                UserMetadata::UserEvmAddress(user_evm_address) => {
                    res.evm_address = Some(user_evm_address.evm_address);
                }
                UserMetadata::UserTelegram(user_telegram) => {
                    res.telegram = Some(user_telegram.telegram_raw);
                }
            }
        }
        res
    }
}
