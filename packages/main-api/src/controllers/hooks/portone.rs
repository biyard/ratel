use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
#[allow(dead_code)]
pub struct PortoneRequest {
    pub payment_id: String,
    pub status: String,
    pub tx_id: String,
}

pub async fn portone_handler(
    State(AppState { .. }): State<AppState>,
    NoApi(headers): NoApi<HeaderMap>,
    Json(req): Json<serde_json::Value>,
) -> Result<()> {
    notify!(
        "Incomming PortOne hook: {:?} with headers {:?}",
        req,
        headers
    );
    // TODO: Implement the handler logic here

    Ok(())
}
