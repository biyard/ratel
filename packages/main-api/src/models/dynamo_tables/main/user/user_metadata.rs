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
