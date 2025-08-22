use std::sync::Arc;

use aws_sdk_rdsdata::{
    Client as RdsDataClient,
    operation::execute_statement::ExecuteStatementOutput,
    types::{RecordsFormatType, SqlParameter},
};
use serde::de::DeserializeOwned;

use dto::{Error, Result};

pub struct RdsClient {
    pub client: RdsDataClient,
    pub resource_arn: String,
    pub secret_arn: String,
    pub database_name: String,
}

impl RdsClient {
    pub fn new(
        client: RdsDataClient,
        resource_arn: &str,
        secret_arn: &str,
        database_name: &str,
    ) -> Arc<Self> {
        Arc::new(RdsClient {
            client,
            resource_arn: resource_arn.to_string(),
            secret_arn: secret_arn.to_string(),
            database_name: database_name.to_string(),
        })
    }

    pub async fn execute_statement(
        &self,
        sql: &str,
        parameters: Option<Vec<SqlParameter>>,
    ) -> Result<ExecuteStatementOutput> {
        let client = self.client.clone();
        let resource_arn = self.resource_arn.clone();
        let secret_arn = self.secret_arn.clone();
        let database_name = self.database_name.clone();
        client
            .execute_statement()
            .resource_arn(resource_arn)
            .secret_arn(secret_arn)
            .database(database_name)
            .sql(sql)
            .set_parameters(parameters)
            .format_records_as(RecordsFormatType::Json)
            .send()
            .await
            .map_err(|e| {
                tracing::debug!("Failed to execute statement: {:?}", e);
                Error::ServerError(e.to_string())
            })
    }

    pub async fn query<T>(&self, sql: &str, parameters: Option<Vec<SqlParameter>>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let result = self.execute_statement(sql, parameters).await?;

        if let Some(formatted_records) = result.formatted_records() {
            tracing::debug!("Formatted records: {}", formatted_records);

            let json_value: serde_json::Value = serde_json::from_str(formatted_records)
                .map_err(|e| Error::ServerError(format!("Failed to parse JSON: {}", e)))?;

            if let serde_json::Value::Array(array) = json_value {
                let mut items = Vec::new();
                for item in array {
                    let parsed_item: T = serde_json::from_value(item).map_err(|e| {
                        Error::ServerError(format!("Failed to deserialize record: {}", e))
                    })?;
                    items.push(parsed_item);
                }
                return Ok(items);
            }

            return Err(Error::ServerError(
                "Expected JSON array from formatted_records".to_string(),
            ));
        }

        let records = result.records();
        if records.is_empty() {
            return Ok(Vec::new());
        }

        let mut items = Vec::new();
        for record in records {
            let json_record = rds_record_to_json(record)?;
            let item: T = serde_json::from_value(json_record)
                .map_err(|e| Error::ServerError(format!("Failed to deserialize record: {}", e)))?;
            items.push(item);
        }

        Ok(items)
    }

    pub async fn query_one<T>(&self, sql: &str, parameters: Option<Vec<SqlParameter>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut items = self.query::<T>(sql, parameters).await?;
        items.pop().ok_or(Error::NotFound)
    }

    pub async fn query_optional<T>(
        &self,
        sql: &str,
        parameters: Option<Vec<SqlParameter>>,
    ) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let items = self.query::<T>(sql, parameters).await?;
        Ok(items.into_iter().next())
    }

    pub async fn insert(&self, sql: &str, parameters: Option<Vec<SqlParameter>>) -> Result<i64> {
        let result = self.execute_statement(sql, parameters).await?;
        Ok(result.number_of_records_updated())
    }

    pub async fn insert_returning<T>(
        &self,
        sql: &str,
        parameters: Option<Vec<SqlParameter>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let result = self.execute_statement(sql, parameters).await;
        tracing::debug!("Insert result: {:?}", result);
        let result = result?;
        if let Some(formatted_records) = result.formatted_records() {
            tracing::debug!("Formatted records: {}", formatted_records);

            let json_value: serde_json::Value = serde_json::from_str(formatted_records)
                .map_err(|e| Error::ServerError(format!("Failed to parse JSON: {}", e)))?;

            if let serde_json::Value::Array(array) = json_value {
                if let Some(first_item) = array.into_iter().next() {
                    let parsed_item: T = serde_json::from_value(first_item).map_err(|e| {
                        Error::ServerError(format!("Failed to deserialize record: {}", e))
                    })?;
                    return Ok(parsed_item);
                }
            }

            return Err(Error::ServerError(
                "Insert operation returned no data".to_string(),
            ));
        }

        let records = result.records();

        if records.is_empty() {
            return Err(Error::ServerError(
                "Insert operation returned no data".to_string(),
            ));
        }

        let json_record = rds_record_to_json(&records[0])?;
        let item: T = serde_json::from_value(json_record).map_err(|e| {
            Error::ServerError(format!("Failed to deserialize inserted record: {}", e))
        })?;

        Ok(item)
    }
    pub async fn insert_many_returning<T>(
        &self,
        sql: &str,
        parameters: Option<Vec<SqlParameter>>,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        self.query(sql, parameters).await
    }
}

fn rds_record_to_json(record: &[aws_sdk_rdsdata::types::Field]) -> Result<serde_json::Value> {
    use aws_sdk_rdsdata::types::Field;
    use serde_json::{Map, Value};

    let mut map = Map::new();

    for (index, field) in record.iter().enumerate() {
        let key = format!("column_{}", index);
        let value = match field {
            Field::StringValue(s) => Value::String(s.clone()),
            Field::LongValue(l) => Value::Number(serde_json::Number::from(*l)),
            Field::DoubleValue(d) => {
                if let Some(num) = serde_json::Number::from_f64(*d) {
                    Value::Number(num)
                } else {
                    Value::Null
                }
            }
            Field::BooleanValue(b) => Value::Bool(*b),
            Field::IsNull(_) => Value::Null,
            Field::BlobValue(blob) => {
                let encoded = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    blob.as_ref(),
                );
                Value::String(encoded)
            }
            _ => Value::Null,
        };
        map.insert(key, value);
    }

    Ok(Value::Object(map))
}
