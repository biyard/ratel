use aws_sdk_dynamodb::{
    model::AttributeValue,
    Client as DynamoDbClient,
};
use bdk::prelude::*;
use dto::{
    Error, Result,
    by_axum::axum::{Json, extract::{Query, State}},
    *,
};
use std::collections::HashMap;

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
        .index_name("GSI1") // Assuming GSI1 is the index for news items
        .key_condition_expression("GSI1PK = :pk")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S("NEWS#ALL".to_string()),
        )
        .limit(limit as i32)
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
            let news_id = item.get("PK")
                .and_then(|v| v.as_s().ok())
                .and_then(|s| s.strip_prefix("NEWS#"))
                .and_then(|s| s.parse::<i64>().ok())?;
                
            let created_at = item.get("created_at")
                .and_then(|v| v.as_n().ok())
                .and_then(|s| s.parse::<i64>().ok())?;
                
            let updated_at = item.get("updated_at")
                .and_then(|v| v.as_n().ok())
                .and_then(|s| s.parse::<i64>().ok())?;
                
            let title = item.get("title")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string())?;
                
            let summary = item.get("summary")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string());
                
            let image_url = item.get("image_url")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string());
                
            let url = item.get("url")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string());
                
            let source = item.get("source")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string());
                
            Some(NewsSummary {
                id: news_id,
                created_at,
                updated_at,
                title,
                summary,
                image_url,
                url,
                source,
            })
        })
        .collect();

    Ok(Json(items))
}
