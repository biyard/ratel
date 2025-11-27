use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
#[allow(dead_code)]
pub struct PortoneRequest {
    pub payment_id: String,
    pub status: String,
    pub tx_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct PortoneResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn portone_handler(
    State(AppState { .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Json(req): Json<PortoneRequest>,
) -> Result<Json<PortoneResponse>> {
    warn!("Handling request: {:?}", req);
    // TODO: Implement the handler logic here

    unimplemented!()
}
