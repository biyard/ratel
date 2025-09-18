// use aws_config::Region;
// use aws_sdk_dynamodb::{Client, Config, config::Credentials, types::AttributeValue};
// use dto::{Error, Result};
// use std::collections::HashMap;

// use crate::config;

// #[derive(Debug, Clone)]
// pub struct DynamoClient {
//     pub client: Client,
//     pub table_name: String,
// }
// impl DynamoClient {
//     pub fn new(table_name: &str) -> Self {
//         let conf = config::get();

//         // Check for local development
//         let mut builder = Config::builder();

//         if let Some(dynamo_url) = option_env!("AWS_ENDPOINT_URL_DYNAMODB") {
//             if dynamo_url.contains("localhost") {
//                 // Use test credentials for localhost
//                 builder = builder
//                     .credentials_provider(Credentials::new(
//                         "test",
//                         "test",
//                         None,
//                         None,
//                         "dynamo",
//                     ))
//                     .endpoint_url(dynamo_url);
//             } else {
//                 // Use production credentials
//                 builder = builder
//                     .credentials_provider(Credentials::new(
//                         conf.aws.access_key_id,
//                         conf.aws.secret_access_key,
//                         None,
//                         None,
//                         "dynamo",
//                     ));
//             }
//         } else {
//             // Use production credentials
//             builder = builder
//                 .credentials_provider(Credentials::new(
//                     conf.aws.access_key_id,
//                     conf.aws.secret_access_key,
//                     None,
//                     None,
//                     "dynamo",
//                 ));
//         }

//         let aws_config = builder
//             .region(Region::new(conf.aws.region))
//             .behavior_version_latest()
//             .build();

//         let client = Client::from_conf(aws_config);

//         Self {
//             client,
//             table_name: table_name.to_string(),
//         }
//     }

//     // Put item with just HashMap<String, AttributeValue>
//     pub async fn put_item(&self, item: HashMap<String, AttributeValue>) -> Result<()> {
//         self.client
//             .put_item()
//             .table_name(&self.table_name)
//             .set_item(Some(item))
//             .send()
//             .await
//             .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//         Ok(())
//     }

//     // Get item by primary key and sort key
//     pub async fn get_item(
//         &self,
//         pk: &str,
//         pk_value: &str,
//         sk: Option<(&str, &str)>,
//     ) -> Result<Option<HashMap<String, AttributeValue>>> {
//         let mut key_map = HashMap::new();
//         key_map.insert(pk.to_string(), AttributeValue::S(pk_value.to_string()));

//         if let Some((sk_name, sk_value)) = sk {
//             key_map.insert(sk_name.to_string(), AttributeValue::S(sk_value.to_string()));
//         }

//         let resp = self
//             .client
//             .get_item()
//             .table_name(&self.table_name)
//             .set_key(Some(key_map))
//             .send()
//             .await
//             .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//         Ok(resp.item)
//     }

//     // Query by GSI
//     pub async fn query_gsi(
//         &self,
//         index_name: &str,
//         key_condition: &str,
//         expression_values: HashMap<String, AttributeValue>,
//     ) -> Result<Vec<HashMap<String, AttributeValue>>> {
//         let resp = self
//             .client
//             .query()
//             .table_name(&self.table_name)
//             .index_name(index_name)
//             .key_condition_expression(key_condition)
//             .set_expression_attribute_values(Some(expression_values))
//             .send()
//             .await
//             .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//         Ok(resp.items.unwrap_or_default())
//     }

//     // Transaction write for atomic operations
//     pub async fn transact_write(
//         &self,
//         put_items: Vec<HashMap<String, AttributeValue>>,
//     ) -> Result<()> {
//         use aws_sdk_dynamodb::types::{Put, TransactWriteItem};

//         let mut transact_items = Vec::new();

//         for item in put_items {
//             let put = Put::builder()
//                 .table_name(&self.table_name)
//                 .set_item(Some(item))
//                 .build()
//                 .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//             transact_items.push(TransactWriteItem::builder().put(put).build());
//         }

//         self.client
//             .transact_write_items()
//             .set_transact_items(Some(transact_items))
//             .send()
//             .await
//             .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//         Ok(())
//     }

//     // Update item by primary key and sort key with expression
//     pub async fn update_item(
//         &self,
//         pk: &str,
//         pk_value: &str,
//         sk: Option<(&str, &str)>,
//         update_expression: &str,
//         expression_values: HashMap<String, AttributeValue>,
//     ) -> Result<()> {
//         let mut key_map = HashMap::new();
//         key_map.insert(pk.to_string(), AttributeValue::S(pk_value.to_string()));

//         if let Some((sk_name, sk_value)) = sk {
//             key_map.insert(sk_name.to_string(), AttributeValue::S(sk_value.to_string()));
//         }

//         self.client
//             .update_item()
//             .table_name(&self.table_name)
//             .set_key(Some(key_map))
//             .update_expression(update_expression)
//             .set_expression_attribute_values(Some(expression_values))
//             .send()
//             .await
//             .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//         Ok(())
//     }

//     // Delete item by primary key and sort key
//     pub async fn delete_item(
//         &self,
//         pk: &str,
//         pk_value: &str,
//         sk: Option<(&str, &str)>,
//     ) -> Result<()> {
//         let mut key_map = HashMap::new();
//         key_map.insert(pk.to_string(), AttributeValue::S(pk_value.to_string()));

//         if let Some((sk_name, sk_value)) = sk {
//             key_map.insert(sk_name.to_string(), AttributeValue::S(sk_value.to_string()));
//         }

//         self.client
//             .delete_item()
//             .table_name(&self.table_name)
//             .set_key(Some(key_map))
//             .send()
//             .await
//             .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//         Ok(())
//     }

//     // Check if item exists by primary key and sort key
//     pub async fn item_exists(
//         &self,
//         pk: &str,
//         pk_value: &str,
//         sk: Option<(&str, &str)>,
//     ) -> Result<bool> {
//         let item = self.get_item(pk, pk_value, sk).await?;
//         Ok(item.is_some())
//     }

//     // Put item with condition (e.g., "attribute_not_exists(PK)" for create-only)
//     pub async fn put_item_conditional(
//         &self,
//         item: HashMap<String, AttributeValue>,
//         condition_expression: &str,
//     ) -> Result<()> {
//         self.client
//             .put_item()
//             .table_name(&self.table_name)
//             .set_item(Some(item))
//             .condition_expression(condition_expression)
//             .send()
//             .await
//             .map_err(|e| Error::DynamoDbError(format!("{:?}", e)))?;

//         Ok(())
//     }
// }

// // Helper functions to create AttributeValues easily
// pub fn string_attr(value: &str) -> AttributeValue {
//     AttributeValue::S(value.to_string())
// }

// pub fn number_attr(value: i64) -> AttributeValue {
//     AttributeValue::N(value.to_string())
// }

// pub fn bool_attr(value: bool) -> AttributeValue {
//     AttributeValue::Bool(value)
// }

use aws_config::Region;
use aws_sdk_dynamodb::{Client, Config, config::Credentials};
use dto::by_types::DatabaseConfig;

use crate::config;

#[derive(Debug, Clone)]
pub struct DynamoClient {
    pub client: Client,
}

impl DynamoClient {
    pub fn new() -> Self {
        let conf = config::get();

        let endpoint = match conf.database {
            DatabaseConfig::DynamoDb { endpoint, .. } => endpoint,
            _ => panic!("DynamoDB config not found"),
        };

        let mut builder = Config::builder()
            .credentials_provider(
                Credentials::builder()
                    .access_key_id(conf.aws.access_key_id)
                    .secret_access_key(conf.aws.secret_access_key)
                    .provider_name("ratel")
                    .build(),
            )
            .region(Region::new(conf.aws.region))
            .behavior_version_latest();

        if let Some(endpoint) = endpoint {
            builder = builder.endpoint_url(endpoint.to_string());
        }
        let config = builder.build();
        let client = Client::from_conf(config);
        Self { client }
    }
}
