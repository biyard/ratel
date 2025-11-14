use aws_sdk_dynamodb::types::TransactWriteItem;

use crate::features::spaces::panels::SpacePanelQuota;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct Keys {
    pub pk: String,
    pub sk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct DeleteAllPanelsRequest {
    pub keys: Vec<Keys>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct DeleteAllPanelsResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn delete_all_panels_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<DeleteAllPanelsRequest>,
) -> Result<Json<DeleteAllPanelsResponse>> {
    tracing::debug!("Handling request: {:?}", req);

    let tx: Vec<TransactWriteItem> = req
        .keys
        .into_iter()
        .map(|Keys { pk, sk }| SpacePanelQuota::delete_transact_write_item(pk, sk))
        .collect();

    transact_write_items!(dynamo.client, tx)?;

    Ok(Json(DeleteAllPanelsResponse {
        status: "All panels deleted successfully".to_string(),
    }))
}
