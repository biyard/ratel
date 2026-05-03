use crate::common::*;
use crate::common::utils::time::get_now_timestamp_millis;

/// Singleton row recording the last successfully applied migration version.
/// Pairs with `Partition::Migration` + `EntityType::LastBackfillVersion` so
/// the entire migration framework state is one (pk, sk).
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct LastBackfillVersion {
    pub pk: Partition,
    pub sk: EntityType,

    pub version: i64,
    pub updated_at: i64,
}

impl LastBackfillVersion {
    pub fn singleton_keys() -> (Partition, EntityType) {
        (Partition::Migration, EntityType::LastBackfillVersion)
    }
}

#[cfg(feature = "server")]
impl LastBackfillVersion {
    /// Atomically advance the stored version from `expected` to `new_version`.
    /// Uses a conditional `UpdateItem` so concurrent replicas can't both
    /// succeed. On the very first run (`expected == 0`), the row may not
    /// exist yet — we permit insert via "attribute_not_exists OR version =
    /// :expected". The macro-generated `updater().execute()` doesn't expose
    /// `condition_expression`, so we issue the request directly.
    pub async fn advance_to(
        cli: &aws_sdk_dynamodb::Client,
        expected: i64,
        new_version: i64,
    ) -> crate::common::Result<()> {
        let (pk, sk) = Self::singleton_keys();
        let now = get_now_timestamp_millis();

        let key = std::collections::HashMap::from([
            (
                "pk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(pk.to_string()),
            ),
            (
                "sk".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()),
            ),
        ]);

        let condition = if expected == 0 {
            "attribute_not_exists(version) OR version = :expected"
        } else {
            "version = :expected"
        };

        let mut values = std::collections::HashMap::new();
        values.insert(
            ":expected".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N(expected.to_string()),
        );
        values.insert(
            ":new_version".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N(new_version.to_string()),
        );
        values.insert(
            ":updated_at".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::N(now.to_string()),
        );

        cli.update_item()
            .table_name(Self::table_name())
            .set_key(Some(key))
            .update_expression("SET version = :new_version, updated_at = :updated_at")
            .condition_expression(condition)
            .set_expression_attribute_values(Some(values))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

        Ok(())
    }
}
