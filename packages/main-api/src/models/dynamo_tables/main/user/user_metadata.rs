use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
#[dynamo(
    pk_prefix = "EMAIL",
    sk_prefix = "AA",
    index = "gsi1",
    name = "find_by_email"
)]
pub enum UserMetadata {
    User(User),
    UserPrincipal(UserPrincipal),
    UserEvmAddress(UserEvmAddress),
    UserReferralCode(UserReferralCode),
    UserPhoneNumber(UserPhoneNumber),
    UserTelegram(UserTelegram),
    UserTeam(UserTeam),
    UserTeamGroup(UserTeamGroup),
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct UserResponse {
    pub pk: String,
    pub email: String,
    pub nickname: String,
    pub profile_url: String,
    pub description: String,
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
            nickname: user.display_name,
            profile_url: user.profile_url,
            description: user.description,
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
    pub user: UserResponse,

    pub referral_code: Option<String>,
    pub phone_number: Option<String>,
    pub principal: Option<String>,
    pub evm_address: Option<String>,
    pub telegram: Option<String>,
    pub teams: Option<Vec<UserTeamResponse>>,
}

impl From<Vec<UserMetadata>> for UserDetailResponse {
    fn from(items: Vec<UserMetadata>) -> Self {
        let mut res = Self::default();
        for item in items {
            match item {
                UserMetadata::User(user) => {
                    res.user = user.into();
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
                UserMetadata::UserTeam(user_team) => {
                    let team: UserTeamResponse = user_team.into();
                    if res.teams.is_none() {
                        res.teams = Some(vec![]);
                    }
                    res.teams.as_mut().unwrap().push(team);
                }
                _ => {
                    // Skip
                }
            }
        }
        res
    }
}

// impl UserMetadata {
//     pub async fn find_by_email(
//         cli: &aws_sdk_dynamodb::Client,
//         pk: impl std::fmt::Display,
//         sk: Option<impl std::fmt::Display>,
//     ) -> std::result::Result<Vec<Self>, crate::Error2> {
//         let mut key_condition = "#pk = :pk";
//         let mut query = cli
//             .query()
//             .table_name("ratel-local-main")
//             .index_name("gsi1-index")
//             .expression_attribute_names("#pk", "gsi1_pk")
//             .expression_attribute_values(
//                 ":pk",
//                 aws_sdk_dynamodb::types::AttributeValue::S(format!("EMAIL#{}", pk)),
//             );

//         if let Some(sk) = sk {
//             key_condition = "#pk = :pk AND begins_with(#sk, :sk)";
//             query = query
//                 .expression_attribute_names("#sk", "gsi1_sk")
//                 .expression_attribute_values(
//                     ":sk",
//                     aws_sdk_dynamodb::types::AttributeValue::S(format!("AA#{}", sk)),
//                 );
//         }

//         let resp = query
//             .key_condition_expression(key_condition)
//             .send()
//             .await
//             .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

//         let items = resp.items.unwrap_or_default();
//         let ret = items
//             .into_iter()
//             .filter_map(|item| {
//                 serde_dynamo::from_item(item).expect("failed to deserialize UserMetadata")
//             })
//             .collect();

//         Ok(ret)
//     }
// }
