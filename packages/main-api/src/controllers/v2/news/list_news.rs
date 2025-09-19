use aws_sdk_dynamodb::{
    types::AttributeValue,
    Client as DynamoDbClient,
};
use bdk::prelude::*;
use dto::{
    Error, Result,
    by_axum::axum::{Json, extract::{Query, State}},
    *,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, JsonSchema, aide::OperationIo)]
pub struct ListNewsQuery {
    pub limit: Option<i64>,
}

pub async fn list_news_handler(
    State(dynamo_client): State<DynamoDbClient>,
    Query(ListNewsQuery { limit }): Query<ListNewsQuery>,
) -> Result<Json<Vec<NewsSummary>>> {
    let limit = limit.unwrap_or(3).clamp(1, 100) as i32;
    let table_name = std::env::var("DYNAMODB_TABLE_NAME").unwrap_or_else(|_| "ratel-local".to_string());

    let result = dynamo_client
        .query()
        .table_name(&table_name)
        .index_name("GSI1")
        .key_condition_expression("GSI1PK = :pk")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S("NEWS#ALL".to_string()),
        )
        .limit(limit)
        .scan_index_forward(false) 
        .send()
        .await
        .map_err(|e| {
            tracing::error!(error=?e, "Failed to query news from DynamoDB");
            Error::ServerError("failed to query news".into())
        })?;

    let items = result.items.unwrap_or_default()
        .into_iter()
        .filter_map(|item| {
            let html_content = item.get("html_content")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string())?;

            Some(NewsSummary {
                html_content,
            })
        })
        .collect();

    Ok(Json(items))
}