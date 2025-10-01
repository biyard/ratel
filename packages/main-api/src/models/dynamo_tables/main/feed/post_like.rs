use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PostLike {
    pub pk: Partition,
    pub sk: EntityType,

    // #[dynamo(prefix = "LIKE", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,
    // #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

impl PostLike {
    pub fn new(pk: Partition, User { pk: user_pk, .. }: User) -> Self {
        let created_at = chrono::Utc::now().timestamp();

        Self {
            pk,
            sk: EntityType::PostLike(user_pk.to_string()),
            created_at,
            user_pk,
        }
    }
}

// impl PostLike {
//     pub async fn batch_get(
//         cli: &aws_sdk_dynamodb::Client,
//         keys: Vec<(impl std::fmt::Display, impl std::fmt::Display)>,
//     ) -> Result<Vec<Self>, crate::Error2> {
//         let keys = keys
//             .iter()
//             .map(|key| {
//                 std::collections::HashMap::from([
//                     (
//                         Self::pk_field(),
//                         aws_sdk_dynamodb::types::AttributeValue::S(key.0.to_string()),
//                     ),
//                     (
//                         "sk".to_string(),
//                         aws_sdk_dynamodb::types::AttributeValue::S(key.1.to_string()),
//                     ),
//                 ])
//             })
//             .collect::<Vec<_>>();

//         let keys_and_attributes = aws_sdk_dynamodb::types::KeysAndAttributes::builder()
//             .set_keys(Some(keys))
//             .consistent_read(false)
//             .build()
//             .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

//         let table_name = Self::table_name();

//         let response = cli
//             .batch_get_item()
//             .request_items(table_name, keys_and_attributes)
//             .send()
//             .await
//             .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

//         let items = if let Some(responses) = response.responses() {
//             if let Some(items) = responses.get(table_name) {
//                 serde_dynamo::from_items(items.to_vec())?
//             } else {
//                 vec![]
//             }
//         } else {
//             vec![]
//         };

//         Ok(items)
//     }
// }
