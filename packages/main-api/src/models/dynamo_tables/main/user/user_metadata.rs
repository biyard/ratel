use crate::features::membership::{UserMembership, UserMembershipResponse};

use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
#[dynamo(pk_prefix = "EMAIL", index = "gsi3", name = "find_by_email")]
pub enum UserMetadata {
    User(User),
    UserRefreshToken(UserRefreshToken),
    UserNotification(UserNotification),
    UserPrincipal(UserPrincipal),
    UserEvmAddress(UserEvmAddress),
    UserReferralCode(UserReferralCode),
    UserPhoneNumber(UserPhoneNumber),
    UserTelegram(UserTelegram),
    UserTeam(UserTeam),
    UserTeamGroup(UserTeamGroup),
    UserMembership(UserMembership),
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct UserResponse {
    pub pk: String,
    pub email: String,
    pub nickname: String,
    pub profile_url: String,
    pub description: String,
    pub user_type: u8,
    pub username: String,

    pub followers_count: i64,
    pub followings_count: i64,

    pub theme: u8,
    pub point: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            pk: user.pk.to_string(),
            email: user.email,
            nickname: user.display_name,
            profile_url: user.profile_url,
            description: user.description,
            user_type: user.user_type as u8,
            followers_count: user.followers_count,
            followings_count: user.followings_count,
            theme: user.theme as u8,
            point: user.points,
            username: user.username,
        }
    }
}
#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct UserDetailResponse {
    #[serde(flatten)]
    pub user: UserResponse,

    pub referral_code: Option<String>,
    pub phone_number: Option<String>,
    pub principal: Option<String>,
    pub evm_address: Option<String>,
    //FIXME: Change Telegram Model
    // pub telegram_id: Option<i64>,
    pub teams: Option<Vec<UserTeamResponse>>,
    pub membership: Option<UserMembershipResponse>,
    pub is_identified: bool,
    pub has_billing_key: bool,
}

impl From<Vec<UserMetadata>> for UserDetailResponse {
    fn from(items: Vec<UserMetadata>) -> Self {
        let mut res = Self::default();
        for item in items {
            match item {
                UserMetadata::User(user) => {
                    res.user = user.into();
                }
                UserMetadata::UserNotification(_) => {}
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
                // UserMetadata::UserTelegram(user_telegram) => {
                //     res.telegram = Some(user_telegram.telegram_raw);
                // }
                UserMetadata::UserTeam(user_team) => {
                    let team: UserTeamResponse = user_team.into();
                    if res.teams.is_none() {
                        res.teams = Some(vec![]);
                    }
                    res.teams.as_mut().unwrap().push(team);
                }
                UserMetadata::UserMembership(membership) => {
                    res.membership = Some(membership.into());
                }
                UserMetadata::UserTeamGroup(_)
                | UserMetadata::UserTelegram(_)
                | UserMetadata::UserRefreshToken(_) => {
                    // Skip
                }
            }
        }
        res
    }
}
